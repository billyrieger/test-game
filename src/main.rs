use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 480.;
const PIXELS_PER_METER: f32 = 100.;
const GRID_SIZE: f32 = 16.;

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
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems((setup.on_startup(), center_camera_on_player.after(move_player), move_player))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("testing.ldtk"),
        transform: Transform::from_xyz(-WIDTH / 2., -HEIGHT / 2., 0.).with_scale(Vec3::splat(1.)),
        ..default()
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

fn center_camera_on_player(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = q_camera.single_mut();
    if let Ok(player_transform) = q_player.get_single() {
        camera_transform.translation.x = player_transform.translation.x - WIDTH / 2.;
        camera_transform.translation.y = player_transform.translation.y - HEIGHT / 2.;
    }
}

fn move_player(input: Res<Input<KeyCode>>, mut q_player: Query<&mut Transform, With<Player>>) {
    for mut transform in q_player.iter_mut() {
        if input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
            transform.translation.x -= GRID_SIZE;
        }
        else if input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
            transform.translation.x += GRID_SIZE;
        }
        else if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
            transform.translation.y += GRID_SIZE;
        }
        else if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
            transform.translation.y -= GRID_SIZE;
        }
    }
}