mod loading;
mod menu;

use bevy::prelude::*;

pub const WIDTH: f32 = 720.;
pub const HEIGHT: f32 = 576.;

#[derive(States, Clone, Default, Debug, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(loading::LoadingPlugin)
            .add_plugin(menu::MenuPlugin);
    }
}
