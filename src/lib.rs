#![allow(clippy::type_complexity)]

mod loading;
mod action;

use crate::action::ActionPlugin;
use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Parsing,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
            .add_state::<GameState>()
            .add_plugins((
                LoadingPlugin,
                ActionPlugin,
            ));
        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct CharacterName(String);

#[derive(Component, Clone, Eq, Hash, PartialEq, Debug)]
enum CharacterState {
    Idle,
    Walk,
    Action(String),
    Hit,
}

impl ToString for CharacterState {
    fn to_string(&self) -> String {
        match self {
            CharacterState::Idle => "idle".to_string(),
            CharacterState::Walk => "walk".to_string(),
            CharacterState::Hit => "hit".to_string(),
            CharacterState::Action(action) => action.clone(),
        }
    }
}

#[derive(Event, Debug)]
enum GameEvent {
    Idle(u32),
    Left(u32),
    Right(u32),
    Action(u32, String),
    Stop(u32),
    Hit(u32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rectbox {
    pub min: Vec2,
    pub max: Vec2,
}

//攻击盒
#[derive(Component, Deref, DerefMut, Debug)]
pub struct Hitbox(pub Rectbox);

//受击盒
#[derive(Component, Deref, DerefMut, Debug)]
pub struct Hurtbox(pub Rectbox);

//格挡盒
#[derive(Component, Deref, DerefMut, Debug)]
pub struct Blockbox(pub Rectbox);

#[derive(Component, PartialEq, Deref, DerefMut, Copy, Clone, Debug)]
pub struct UID(u32);

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub repeat: bool,
}

#[derive(Component, Clone, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(bevy::reflect::TypeUuid, bevy::reflect::TypePath, Resource, Deserialize, Serialize, Debug)]
#[uuid = "1afacdb5-f62c-4f3d-a3af-d3faec98c45f"]
pub struct Character {
    pub name: String,
    pub actions: HashMap<String, Action>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Action {
    pub duration: f32,
    pub repeat: bool,
    pub atlas: ActionAtlas,
    pub frames: Vec<ActionFrame>,
    pub next_action: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActionAtlas {
    pub width: usize,
    pub height: usize,
    pub columns: usize,
    pub rows: usize,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActionFrame {
    pub id: u32,
    pub stage: ActionStage,
    pub hitbox: Option<Rectbox>,
    pub hurtbox: Rectbox,
    pub blockbox: Option<Rectbox>,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub enum ActionStage {
    Startup,
    Active,
    Recovery,
}