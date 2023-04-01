use crate::{loading::GameAssets, GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                ..default()
            })
            .add_system(setup.in_schedule(OnEnter(GameState::InGame)));
    }
}

// =================
// ==== SYSTEMS ====
// =================

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: game_assets.levels.clone(),
        level_set: LevelSet {
            iids: [
                "b9b96ae0-c640-11ed-bbe1-13f03a4ef032",
                "836a3cd0-c640-11ed-bd91-1badb27c06af",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
        transform: Transform::from_scale(Vec3::splat(1.)).with_translation(Vec3::new(
            -crate::WIDTH / 2.,
            -crate::HEIGHT / 2.,
            0.,
        )),
        ..default()
    });
}
