use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (test_game::WIDTH, test_game::HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(test_game::GamePlugin)
        .run();
}
