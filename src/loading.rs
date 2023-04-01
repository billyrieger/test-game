use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading);
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "textures/px.png")]
    pub px_texture: Handle<Image>,
    #[asset(path = "fonts/Kenney Pixel.ttf")]
    pub menu_font: Handle<Font>,
    #[asset(path = "levels/testing.ldtk")]
    pub levels: Handle<bevy_ecs_ldtk::LdtkAsset>,
}
