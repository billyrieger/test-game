use bevy::prelude::*;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("goat_circle.png"),
        ..default()
    });
}
