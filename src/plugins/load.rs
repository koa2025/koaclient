use crate::{MyAssets, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup)
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Init)
            )
            .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {}