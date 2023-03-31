use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_particle_systems::ParticleSystemBundle;
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;

use config::constants::*;

const PIXELS_PER_METER: f32 = 16.;
const GRID_SIZE: f32 = 16.;

mod config {
    use bevy::prelude::*;

    pub mod constants {
        pub const WIDTH: f32 = 640.;
        pub const HEIGHT: f32 = 480.;
        pub const HALF_WIDTH: f32 = WIDTH / 2.;
        pub const HALF_HEIGHT: f32 = HEIGHT / 2.;
    }

    pub struct ConfigPlugin;

    impl Plugin for ConfigPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: (constants::WIDTH, constants::HEIGHT).into(),
                            ..default()
                        }),
                        ..default()
                    }),
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugin(config::ConfigPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            ..default()
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(bevy_particle_systems::ParticleSystemPlugin)
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<DoorBundle>("Door")
        .register_ldtk_entity::<PortalBundle>("Portal")
        // .register_ldtk_int_cell::<WallBundle>(2)
        .add_systems((
            setup.on_startup(),
            spawn_particles,
            move_player,
            add_wall_colliders,
            debug_query,
        ))
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Root;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    use bevy_particle_systems::*;
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("testing.ldtk"),
            level_set: LevelSet {
                iids: [
                    "b9b96ae0-c640-11ed-bbe1-13f03a4ef032",
                    "836a3cd0-c640-11ed-bd91-1badb27c06af",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            },
            transform: Transform::from_translation(Vec3::new(-HALF_WIDTH, -HALF_HEIGHT, 0.)),
            ..default()
        })
        .with_children(|parent| {});
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    #[with(player_physics)]
    #[bundle]
    physics: (RigidBody, Collider, Velocity),
}

fn player_physics(_: &EntityInstance) -> (RigidBody, Collider, Velocity) {
    (
        RigidBody::Dynamic,
        Collider::ball(GRID_SIZE / 2.),
        Velocity::default(),
    )
}

#[derive(Component, Default)]
struct Door;

#[derive(Component)]
struct GameEntity(EntityInstance);

impl From<&EntityInstance> for GameEntity {
    fn from(value: &EntityInstance) -> Self {
        // println!("{:#?}", value);
        Self(value.clone())
    }
}

#[derive(Bundle, LdtkEntity)]
struct DoorBundle {
    door: Door,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    #[from_entity_instance]
    entity_instance: GameEntity,
    #[with(door_physics)]
    #[bundle]
    physics: (RigidBody, Collider),
}

fn door_physics(_: &EntityInstance) -> (RigidBody, Collider) {
    (
        RigidBody::Fixed,
        Collider::cuboid(GRID_SIZE / 2., GRID_SIZE / 2.),
    )
}

#[derive(Component, Default)]
struct Portal;

#[derive(Bundle, LdtkEntity)]
struct PortalBundle {
    portal: Portal,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    rigid_body: RigidBody,
    collider: Collider,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(GRID_SIZE / 2., GRID_SIZE / 2.),
        }
    }
}

fn add_wall_colliders(
    mut commands: Commands,
    q_walls: Query<(Entity, &IntGridCell), Added<IntGridCell>>,
) {
    for (entity, cell) in &q_walls {
        match cell.value {
            // walls & ceilings
            2 | 3 => {
                commands.entity(entity).insert(WallBundle::default());
            }
            // doors
            4 => {}
            _ => continue,
        }
    }
}

fn move_player(input: Res<Input<KeyCode>>, mut q_player: Query<&mut Velocity, With<Player>>) {
    for mut velocity in q_player.iter_mut() {
        velocity.linvel = Vec2::ZERO;
        if input.any_pressed([KeyCode::Left, KeyCode::A]) {
            velocity.linvel.x -= 160.;
        }
        if input.any_pressed([KeyCode::Right, KeyCode::D]) {
            velocity.linvel.x += 160.;
        }
        if input.any_pressed([KeyCode::Down, KeyCode::S]) {
            velocity.linvel.y -= 160.;
        }
        if input.any_pressed([KeyCode::Up, KeyCode::W]) {
            velocity.linvel.y += 160.;
        }
    }
}

fn spawn_particles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_player: Query<Entity, Added<Player>>,
) {
    use bevy_particle_systems::*;

    for entity in &q_player {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(ParticleSystemBundle {
                    particle_system: ParticleSystem {
                        max_particles: 50_000,
                        texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                        spawn_rate_per_second: 1000.0.into(),
                        initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                        velocity_modifiers: vec![VelocityModifier::Drag(0.01.into())],
                        lifetime: JitteredValue::jittered(8.0, -2.0..2.0),
                        color: ColorOverTime::Gradient(Curve::new(vec![
                            CurvePoint::new(Color::PURPLE, 0.0),
                            CurvePoint::new(Color::RED, 0.5),
                            CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                        ])),
                        looping: true,
                        system_duration_seconds: 10.0,
                        max_distance: Some(300.0),
                        scale: 2.0.into(),
                        bursts: vec![
                            ParticleBurst::new(0.0, 1000),
                            ParticleBurst::new(2.0, 1000),
                            ParticleBurst::new(4.0, 1000),
                            ParticleBurst::new(6.0, 1000),
                            ParticleBurst::new(8.0, 1000),
                        ],
                        space: ParticleSpace::World,
                        ..ParticleSystem::default()
                    },
                    transform: Transform::from_xyz(0., 0., 10.),
                    ..ParticleSystemBundle::default()
                })
                .insert(Playing);
        });
    }
}

fn debug_query(
    rapier_ctx: Res<RapierContext>,
    input: Res<Input<KeyCode>>,
    q_debug: Query<(Entity, &Collider, &Transform)>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (e1, _c1, _t1) in &q_debug {
            for (e2, _c2, _t2) in &q_debug {
                if e1 == e2 {
                    continue;
                }
                println!("{e1:?}, {e2:?}");
                dbg!(rapier_ctx.contact_pair(e1, e2).map(|_| ()));
            }
        }
    }
}
