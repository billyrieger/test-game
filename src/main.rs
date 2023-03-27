use bevy::{prelude::*, transform::TransformSystem};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 480.;
const PIXELS_PER_METER: f32 = 100.;
const GRID_SIZE: f32 = 16.;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum MySystemSet {
    Early,
    Normal,
    Late,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH, HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(TweeningPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(LdtkPlugin)
        .configure_set(
            MySystemSet::Late
                .in_base_set(CoreSet::PostUpdateFlush)
                .before(CoreSet::Last)
                .after(CoreSet::PostUpdate),
        )
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<DoorBundle>("Door")
        .add_systems((setup.on_startup(), move_player))
        .add_system(center_camera_on_player.in_base_set(CoreSet::PostUpdate))
        .run();
}

#[derive(Component)]
struct Root;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Root, SpatialBundle::default()))
        .with_children(|parent| {
            parent.spawn((Camera2dBundle::default(), MainCamera));
            parent.spawn(LdtkWorldBundle {
                ldtk_handle: asset_server.load("testing.ldtk"),
                transform: Transform::from_scale(Vec3::splat(3.)),
                ..default()
            });
        });
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

#[derive(Component, Default)]
struct Door;

#[derive(Component)]
struct GameEntity(EntityInstance);

impl From<&EntityInstance> for GameEntity {
    fn from(value: &EntityInstance) -> Self {
        println!("{:#?}", value);
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
}

fn center_camera_on_player(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_player: Query<&GlobalTransform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = q_camera.single_mut();
    if let Ok(player_transform) = q_player.get_single() {
        camera_transform.translation.x = player_transform.translation().x;
        camera_transform.translation.y = player_transform.translation().y;
    }
}

fn move_player(input: Res<Input<KeyCode>>, mut q_player: Query<&mut Transform, With<Player>>) {
    for mut transform in q_player.iter_mut() {
        if input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
            transform.translation.x -= GRID_SIZE;
        } else if input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
            transform.translation.x += GRID_SIZE;
        } else if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
            transform.translation.y += GRID_SIZE;
        } else if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
            transform.translation.y -= GRID_SIZE;
        }
        // transform.translation.z = 1.
    }
}
