mod level;
mod loading;
mod menu;

use bevy::prelude::*;

pub const WIDTH: f32 = 640.;
pub const HEIGHT: f32 = 480.;

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
            .add_plugin(level::LevelPlugin)
            .add_plugin(menu::MenuPlugin)
            .add_plugin(loading::LoadingPlugin)
            .add_system(setup.on_startup());
    }
}

// ==== COMPONENTS ====

#[derive(Component)]
struct MainCamera;

// ==== SYSTEMS ====

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}
