use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (test_game::WIDTH, test_game::HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(test_game::GamePlugin)
        .run();
}
