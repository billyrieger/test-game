use bevy::prelude::*;
use bevy_tweening::*;

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
        .add_plugin(TweeningPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        std::time::Duration::from_secs(1),
        lens::TransformPositionLens {
            start: Vec3::new(-100., 100., 0.),
            end: Vec3::new(100., -100., 0.),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("goat_circle.png"),
            ..default()
        })
        .insert(Animator::new(tween));
}
