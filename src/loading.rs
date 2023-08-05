use crate::{Character, CharacterState, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), setup)
            .add_plugins(YamlAssetPlugin::<Character>::new(&["character.yaml"]))
            .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Parsing))
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::Loading,
                "character.assets.ron",
            )
            .add_collection_to_loading_state::<_, CharacterAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
            // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
            .add_systems(OnEnter(GameState::Parsing), parsing_assets)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn parsing_assets(
    character_assets: Res<CharacterAssets>,
    characters: Res<Assets<Character>>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    mut character_texture_atlas: ResMut<CharacterTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (name, handle) in character_assets.characters.iter() {
        let character = characters.get(handle).unwrap();
        info!("parsing_assets: {}", character.name);
        let mut action_texture_atlas = HashMap::<String, Handle<TextureAtlas>>::new();
        for (action_name, action) in character.actions.iter() {
            let texture_handle = character_assets.textures.get(&action.atlas.path).unwrap();
            let texture_atlas = texture_atlas_assets.add(
                TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(action.atlas.width as f32, action.atlas.height as f32), action.atlas.columns, action.atlas.rows, None, None)
            );
            action_texture_atlas.insert(action_name.clone(), texture_atlas);
        }
        character_texture_atlas.insert(character.name.clone(), action_texture_atlas);
        next_state.set(GameState::Playing);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(key = "characters", collection(typed, mapped))]
    pub characters: HashMap<String, Handle<Character>>,

    #[asset(key = "textures", collection(typed, mapped))]
    pub textures: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct CharacterTextureAtlas(HashMap<String, HashMap<String, Handle<TextureAtlas>>>);