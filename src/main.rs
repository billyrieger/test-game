use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use spew::prelude::*;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;
const PIXELS_PER_METER: f32 = 100.;
const BALL_DIAMETER_PX: f32 = 64.;

#[derive(Debug, PartialEq, Eq)]
enum Spawnable {
    Ball,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TweeningPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(SpewPlugin::<Spawnable, Transform>::default())
        .add_system(setup.on_startup())
        .add_system(spawn_ball_on_mouse_click)
        .add_spawner((Spawnable::Ball, spawn_ball))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn spawn_ball_on_mouse_click(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut spawn_events: EventWriter<SpawnEvent<Spawnable, Transform>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_events.send(SpawnEvent::with_data(
                Spawnable::Ball,
                Transform::from_xyz(position.x, position.y, 0.),
            ));
        }
    }
}

fn spawn_ball(
    In(transform): In<Transform>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("goat_circle.png"),
            transform,
            ..default()
        })
        .insert((
            RigidBody::Dynamic,
            Collider::ball(0.5 * BALL_DIAMETER_PX),
        ));
}
