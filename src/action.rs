use bevy::math::vec2;
use bevy::prelude::*;

use crate::{Direction, AnimationIndices, AnimationTimer, CharacterState, GameEvent, GameState, UID, Character, CharacterName, ActionStage, Hitbox, Hurtbox, Blockbox, Rectbox};
use crate::loading::{CharacterAssets, CharacterTextureAtlas};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, input.run_if(in_state(GameState::Playing)))
            .add_systems(Update, state.run_if(in_state(GameState::Playing)))
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, action.run_if(in_state(GameState::Playing)))
            .add_systems(Update, damage.run_if(in_state(GameState::Playing)))
            .add_systems(Update, hurtbox.run_if(in_state(GameState::Playing)))
            .add_systems(Update, animation.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut character_texture_atlas: ResMut<CharacterTextureAtlas>,
    character_assets: Res<CharacterAssets>,
    characters: Res<Assets<Character>>,
) {
    let character_name = "skeleton";
    let character = get_character(character_name, &character_assets, &characters);
    let texture_atlas = character_texture_atlas.get(character_name).unwrap().get("idle").unwrap().clone();
    let action = character.actions.get("idle").unwrap();
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas:texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(-200., 0., 0.),
            ..default()
        },
        AnimationIndices { first: 0, last: action.frames.len() - 1, repeat: true },
        AnimationTimer(Timer::from_seconds(action.duration / action.frames.len() as f32, TimerMode::Repeating)),
        CharacterName(character_name.to_string()),
        CharacterState::Idle,
        Direction::Left,
        UID(1),
    ));

    let mut sprite2= TextureAtlasSprite::new(0);
    sprite2.flip_x = true;

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas:texture_atlas.clone(),
            sprite: sprite2,
            transform: Transform::from_xyz(200., 0., 0.),
            ..default()
        },
        AnimationIndices { first: 0, last: action.frames.len() - 1, repeat: true },
        AnimationTimer(Timer::from_seconds(action.duration / action.frames.len() as f32, TimerMode::Repeating)),
        CharacterName(character_name.to_string()),
        CharacterState::Idle,
        Direction::Right,
        UID(2),
    ));
}

fn input(
    input: Res<Input<KeyCode>>,
    mut ew: EventWriter<GameEvent>,
) {
    let mut events = Vec::new();

    if input.pressed(KeyCode::A) {
        events.push(GameEvent::Left(1));
    } else if input.pressed(KeyCode::D) {
        events.push(GameEvent::Right(1));
    }
    if input.pressed(KeyCode::J) {
        events.push(GameEvent::Action(1, "attack".to_string()));
    }
    if input.pressed(KeyCode::K) {
        events.push(GameEvent::Action(1, "block".to_string()));
    }
    // if input.pressed(KeyCode::L) {
    //     events.push(GameEvent::Dodge(1));
    // }
    if events.len() == 0 {
        events.push(GameEvent::Idle(1));
    }
    ew.send_batch(events);
}

fn state(
    mut events: EventReader<GameEvent>,
    mut character_texture_atlas: ResMut<CharacterTextureAtlas>,
    character_assets: Res<CharacterAssets>,
    characters: Res<Assets<Character>>,
    mut query: Query<(Entity, &UID, &mut CharacterState, &CharacterName, &mut Direction, &mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
) {
    for event in events.iter() {
        for (entity, uid, mut state, character_name, mut direction, mut indices, mut timer, mut sprite, mut texture) in &mut query {
            match (event, (*state).clone()) {
                (GameEvent::Idle(u), CharacterState::Walk) => {
                    if *u != uid.0 {
                        continue;
                    }
                    *state = CharacterState::Idle;
                    let action_name = "idle";
                    set_character_action(
                        sprite,
                        texture,
                        indices,
                        timer,
                        character_name,
                        action_name,
                        &character_texture_atlas,
                        &character_assets,
                        &characters,
                    );
                }
                (GameEvent::Left(u) | GameEvent::Right(u), CharacterState::Idle) => {
                    if *u != uid.0 {
                        continue;
                    }
                    *state = CharacterState::Walk;
                    *direction = match event {
                        GameEvent::Left(_) => Direction::Left,
                        GameEvent::Right(_) => Direction::Right,
                        _ => panic!("Invalid state"),
                    };

                    let flip_x = match *direction {
                        Direction::Left => true,
                        Direction::Right => false,
                    };
                    if sprite.flip_x != flip_x {
                        sprite.flip_x = flip_x;
                    }

                    let action_name = "walk";
                    set_character_action(
                        sprite,
                        texture,
                        indices,
                        timer,
                        character_name,
                        action_name,
                        &character_texture_atlas,
                        &character_assets,
                        &characters,
                    );
                }
                (GameEvent::Left(u) | GameEvent::Right(u), CharacterState::Walk) => {
                    if *u != uid.0 {
                        continue;
                    }
                    let dire = match event {
                        GameEvent::Left(_) => Direction::Left,
                        GameEvent::Right(_) => Direction::Right,
                        _ => panic!("Invalid state"),
                    };
                    if *direction != dire {
                        *direction = dire;
                    }
                    let flip_x = match *direction {
                        Direction::Left => true,
                        Direction::Right => false,
                    };
                    if sprite.flip_x != flip_x {
                        sprite.flip_x = flip_x;
                    }
                }
                (GameEvent::Action(u, name), CharacterState::Walk | CharacterState::Idle) => {
                    if *u != uid.0 {
                        continue;
                    }
                    *state = CharacterState::Action(name.clone());
                    let action_name = name.as_str();
                    set_character_action(
                        sprite,
                        texture,
                        indices,
                        timer,
                        character_name,
                        action_name,
                        &character_texture_atlas,
                        &character_assets,
                        &characters,
                    );
                }
                (GameEvent::Action(u, new_action_name), CharacterState::Action(current_action_name)) => {
                    if *u != uid.0 {
                        continue;
                    }
                    let action = get_character(character_name.as_str(), &character_assets, &characters).actions.get(&current_action_name).unwrap();
                    if action.frames[sprite.index].stage == ActionStage::Recovery {
                        let action_name = if new_action_name == &current_action_name { //同一技能触发连招
                            if let Some(next_action_name) = action.next_action.clone() {
                                next_action_name
                            } else {
                                current_action_name.clone()
                            }
                        } else {
                            new_action_name.to_string()
                        };

                        *state = CharacterState::Action(action_name.clone());

                        set_character_action(
                            sprite,
                            texture,
                            indices,
                            timer,
                            character_name,
                            &action_name,
                            &character_texture_atlas,
                            &character_assets,
                            &characters,
                        );
                    }
                }
                (GameEvent::Stop(u), CharacterState::Action(_) | CharacterState::Hit) => {
                    if *u != uid.0 {
                        continue;
                    }
                    *state = CharacterState::Idle;
                    let action_name = "idle";
                    set_character_action(
                        sprite,
                        texture,
                        indices,
                        timer,
                        character_name,
                        action_name,
                        &character_texture_atlas,
                        &character_assets,
                        &characters,
                    );
                }
                (GameEvent::Hit(u), _) => {
                    if *u != uid.0 {
                        continue;
                    }
                    *state = CharacterState::Hit;
                    let action_name = "hit";
                    set_character_action(
                        sprite,
                        texture,
                        indices,
                        timer,
                        character_name,
                        action_name,
                        &character_texture_atlas,
                        &character_assets,
                        &characters,
                    );
                }
                _ => {}
            }
        }
    }
}

fn action(
    mut commands: Commands,
    character_assets: Res<CharacterAssets>,
    characters: Res<Assets<Character>>,
    mut query: Query<(Entity, &CharacterState, &CharacterName, &Transform, &Direction, &TextureAtlasSprite, Option<&mut Hitbox>, Option<&mut Hurtbox>, Option<&mut Blockbox>)>,
) {
    for (entity, state, character_name, transform, direction, sprite, mut hitbox, mut hurtbox, mut blockbox) in query.iter_mut() {
        let action_name = state.to_string();
        let action = get_character(
            character_name.as_str(),
            &character_assets,
            &characters,
        ).actions.get(&action_name).unwrap();
        let frame = action.frames.get(sprite.index).unwrap();

        let (min_x, max_x) = match direction {
            Direction::Right => {
                (frame.hurtbox.min.x, frame.hurtbox.max.x)
            }
            Direction::Left => {
                (frame.hurtbox.min.x * -1., frame.hurtbox.max.x * -1.)
            }
        };

        let current_hurtbox = Hurtbox(Rectbox {
            min: vec2(min_x, frame.hurtbox.min.y),
            max: vec2(max_x, frame.hurtbox.max.y),
        });

        if let Some(mut hurtbox) = hurtbox {
            *hurtbox = current_hurtbox;
        } else {
            commands.entity(entity).insert(current_hurtbox);
        }

        if let Some(ref frame_hitbox) = frame.hitbox {
            let (min_x, max_x) = match direction {
                Direction::Right => {
                    (frame_hitbox.min.x, frame_hitbox.max.x)
                }
                Direction::Left => {
                    (frame_hitbox.min.x * -1., frame_hitbox.max.x * -1.)
                }
            };
            let current_hitbox = Hitbox(Rectbox {
                min: vec2(min_x, frame_hitbox.min.y),
                max: vec2(max_x, frame_hitbox.max.y),
            });
            if let Some(mut hitbox) = hitbox {
                *hitbox = current_hitbox;
            } else {
                commands.entity(entity).insert(current_hitbox);
            }
        } else {
            if let Some(mut hitbox) = hitbox {
                commands.entity(entity).remove::<Hitbox>();
            }
        }

        //blockbox
        if let Some(ref frame_blockbox) = frame.blockbox {
            let (min_x, max_x) = match direction {
                Direction::Right => {
                    (frame_blockbox.min.x, frame_blockbox.max.x)
                }
                Direction::Left => {
                    (frame_blockbox.min.x * -1., frame_blockbox.max.x * -1.)
                }
            };
            let current_blockbox = Blockbox(Rectbox {
                min: vec2(min_x, frame_blockbox.min.y),
                max: vec2(max_x, frame_blockbox.max.y),
            });
            if let Some(mut blockbox) = blockbox {
                *blockbox = current_blockbox;
            } else {
                commands.entity(entity).insert(current_blockbox);
            }
        } else {
            if let Some(mut blockbox) = blockbox {
                commands.entity(entity).remove::<Blockbox>();
            }
        }
    }
}

fn damage(
    mut hitbox_query: Query<(&Transform, &Hitbox, &UID)>,
    mut hurtbox_query: Query<(&Transform, &Hurtbox, &UID)>,
    mut events: EventWriter<GameEvent>,
) {
    for (hit_transform, hitbox, hituid) in hitbox_query.iter_mut() {
        for (hurt_transform, hurtbox, hurtuid) in hurtbox_query.iter_mut() {
            if hituid != hurtuid {
                let x = hit_transform.translation.x;
                let y = hit_transform.translation.y;

                let hit = Rectbox {
                    min: vec2(x + hitbox.min.x, y + hitbox.min.y),
                    max: vec2(x + hitbox.max.x, y + hitbox.max.y),
                };

                let hurt = Rectbox {
                    min: vec2(hurt_transform.translation.x + hurtbox.min.x, hurt_transform.translation.y + hurtbox.min.y),
                    max: vec2(hurt_transform.translation.x + hurtbox.max.x, hurt_transform.translation.y + hurtbox.max.y),
                };

                let x_overlap = hit.min.x <= hurt.max.x && hit.max.x >= hurt.min.x;
                let y_overlap = hit.min.y <= hurt.max.y && hit.max.y >= hurt.min.y;
                if x_overlap && y_overlap {
                    events.send(GameEvent::Hit(hurtuid.0))
                }
            }
        }
    }
}

fn animation(
    time: Res<Time>,
    mut events: EventWriter<GameEvent>,
    mut query: Query<(&UID, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (uid, indices, mut timer, mut sprite) in &mut query {
        if indices.repeat == false && sprite.index == indices.last {
            events.send(GameEvent::Stop(uid.0));
            continue;
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

const SPEED: f32 = 150.0;

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CharacterState, &Direction)>,
) {
    for (mut transform, state, direction) in &mut query {
        if state == &CharacterState::Walk {
            match direction {
                Direction::Left => {
                    transform.translation.x -= SPEED * time.delta_seconds();
                }
                Direction::Right => {
                    transform.translation.x += SPEED * time.delta_seconds();
                }
                _ => panic!("Invalid direction"),
            }
        }
    }
}

fn hurtbox(
    mut query: Query<(&Transform, &Hurtbox, Option<&Hitbox>, Option<&Blockbox>)>,
    mut gizmos: Gizmos,
) {
    for (transform, hurtbox, hitbox, blockbox) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let p_x = x + hurtbox.min.x + (hurtbox.max.x - hurtbox.min.x) / 2.;
        let p_y = y + hurtbox.min.y + (hurtbox.max.y - hurtbox.min.y) / 2.;

        let position = vec2(p_x, p_y);
        let size = vec2(hurtbox.max.x - hurtbox.min.x, hurtbox.max.y - hurtbox.min.y);
        info!("position: {}, size: {}", position, size);
        gizmos.rect_2d(
            position,
            0.,
            size,
            Color::BLUE,
        );

        if let Some(hitbox) = hitbox {
            let p_x = x + hitbox.min.x + (hitbox.max.x - hitbox.min.x) / 2.;
            let p_y = y + hitbox.min.y + (hitbox.max.y - hitbox.min.y) / 2.;

            let position = vec2(p_x, p_y);
            let size = vec2(hitbox.max.x - hitbox.min.x, hitbox.max.y - hitbox.min.y);
            info!("position: {}, size: {}", position, size);
            gizmos.rect_2d(
                position,
                0.,
                size,
                Color::RED,
            );
        }

        if let Some(blockbox) = blockbox {
            let p_x = x + blockbox.min.x + (blockbox.max.x - blockbox.min.x) / 2.;
            let p_y = y + blockbox.min.y + (blockbox.max.y - blockbox.min.y) / 2.;

            let position = vec2(p_x, p_y);
            let size = vec2(blockbox.max.x - blockbox.min.x, blockbox.max.y - blockbox.min.y);
            info!("position: {}, size: {}", position, size);
            gizmos.rect_2d(
                position,
                0.,
                size,
                Color::WHITE,
            );
        }
    }
}

fn get_character<'a, 'b, 'c>(
    name: &'a str,
    character_assets: &'b Res<CharacterAssets>,
    characters: &'c Res<Assets<Character>>,
) -> &'c Character
    where
        'a: 'c, // The lifetime 'a must outlive 'c
{
    let path = format!("characters/{}.character.yaml", name);
    let handle = character_assets.characters.get(&path).unwrap();
    characters.get(handle).unwrap()
}

fn set_character_action<'a, 'b, 'c, 'd>(
    mut sprite: Mut<TextureAtlasSprite>,
    mut texture: Mut<Handle<TextureAtlas>>,
    mut indices: Mut<AnimationIndices>,
    mut timer: Mut<AnimationTimer>,
    character_name: &'a CharacterName,
    action_name: &'b str,
    character_texture_atlas: &'c CharacterTextureAtlas,
    character_assets: &'d Res<CharacterAssets>,
    characters: &'d Res<Assets<Character>>,
) {
    info!("action_name: {}", action_name);
    let action = get_character(
        character_name.as_str(),
        character_assets,
        characters,
    ).actions.get(action_name).unwrap();

    let texture_atlas = character_texture_atlas
        .get(character_name.as_str())
        .unwrap()
        .get(action_name)
        .unwrap()
        .clone();

    sprite.index = 0;
    *texture = texture_atlas.clone();
    *indices = AnimationIndices {
        first: 0,
        last: action.frames.len() - 1,
        repeat: action.repeat,
    };
    *timer = AnimationTimer(Timer::from_seconds(
        action.duration / action.frames.len() as f32,
        TimerMode::Repeating,
    ));
}