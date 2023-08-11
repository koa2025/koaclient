use bevy::math::vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{Direction, AnimationIndices, AnimationTimer, CharacterState, GameEvent, GameState, UID, Character, CharacterName, ActionStage, Hitbox, Hurtbox, Blockbox, Rectbox, OwnerUID, Action, OwnerCharacter, CMD};
use crate::loading::{Characters, CharactersTextureAtlas};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OwnerUID(1))
            .insert_resource(OwnerCharacter("skeleton".to_string()))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, input.run_if(in_state(GameState::Playing)))
            .add_systems(Update, state.run_if(in_state(GameState::Playing)))
            // .add_systems(Update, movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, action.run_if(in_state(GameState::Playing)))
            .add_systems(Update, damage.run_if(in_state(GameState::Playing)))
            .add_systems(Update, hurtbox.run_if(in_state(GameState::Playing)))
            .add_systems(Update, animation.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut characters: Res<Characters>,
    mut characters_texture_atlas: Res<CharactersTextureAtlas>,
) {
    commands
        .spawn(Collider::cuboid(10., 1000.))
        .insert(TransformBundle::from(Transform::from_xyz(-960. / 2., -100., 0.0)));

    commands
        .spawn(Collider::cuboid(10., 1000.))
        .insert(TransformBundle::from(Transform::from_xyz(960. / 2., -100., 0.0)));
    commands
        .spawn((
            Collider::cuboid(1000.0, 20.0),
            TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
            Restitution {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            Friction {
                coefficient: 2.,
                combine_rule: CoefficientCombineRule::Max,
            },
        ));

    let character_name = "skeleton";
    let character = characters.get(character_name).unwrap();
    let action_name = "idle";
    let action = character.actions.get(action_name).unwrap();
    let texture_atlas = characters_texture_atlas.get(character_name).unwrap().get(action_name).unwrap().clone();
    create_character(&mut commands, texture_atlas.clone(), character_name, action, 1);
    create_character(&mut commands, texture_atlas.clone(), character_name, action, 2);
}

fn create_character(commands: &mut Commands, texture_atlas: Handle<TextureAtlas>, character_name: &str, action: &Action, uid: u32) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule_y(15., 15.),
        KinematicCharacterController::default(),
        GravityScale(4.0),
        ColliderMassProperties::Density(2.0),
        LockedAxes::ROTATION_LOCKED,
        Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        // Friction {
        //     coefficient: 0.,
        //     combine_rule: CoefficientCombineRule::Min,
        // },
        SpriteSheetBundle {
            texture_atlas,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        AnimationIndices { first: 0, last: action.frames.len() - 1, repeat: true },
        AnimationTimer(Timer::from_seconds(action.duration / action.frames.len() as f32, TimerMode::Repeating)),
        CharacterName(character_name.to_string()),
        CharacterState::Idle,
        Direction::Left,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
        UID(uid),
    ));
}

fn input(
    input: Res<Input<KeyCode>>,
    mut ew: EventWriter<GameEvent>,
    mut owner: ResMut<OwnerUID>,
    owner_character: Res<OwnerCharacter>,
    characters: Res<Characters>,
) {
    let mut events = Vec::new();

    if input.pressed(KeyCode::Key1) {
        *owner = OwnerUID(1);
    }
    if input.pressed(KeyCode::Key2) {
        *owner = OwnerUID(2);
    }
    if input.pressed(KeyCode::A) {
        events.push(GameEvent::Left(UID(owner.0)));
    } else if input.pressed(KeyCode::D) {
        events.push(GameEvent::Right(UID(owner.0)));
    }
    if input.pressed(KeyCode::J) {
        let character = characters.get(owner_character.0.as_str()).unwrap();
        events.push(GameEvent::Action(UID(owner.0), character.commands.get(&CMD::J).unwrap().clone()));
    }
    if input.pressed(KeyCode::K) {
        let character = characters.get(owner_character.0.as_str()).unwrap();
        events.push(GameEvent::Action(UID(owner.0), character.commands.get(&CMD::K).unwrap().clone()));
    }
    if input.pressed(KeyCode::I) {
        let character = characters.get(owner_character.0.as_str()).unwrap();
        events.push(GameEvent::Action(UID(owner.0), character.commands.get(&CMD::I).unwrap().clone()));
    }
    // if input.pressed(KeyCode::L) {
    //     events.push(GameEvent::Dodge(1));
    // }
    if events.len() == 0 {
        events.push(GameEvent::Idle(UID(owner.0)));
    }
    ew.send_batch(events);
}

fn state(
    mut commands: Commands,
    mut events: EventReader<GameEvent>,
    mut characters: Res<Characters>,
    mut characters_texture_atlas: Res<CharactersTextureAtlas>,
    mut query: Query<(Entity, &UID, &mut Velocity, &mut CharacterState, &CharacterName, &mut Direction, &mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
) {
    for event in events.iter() {
        for (entity, hituid, mut velocity, mut state, character_name, mut direction, mut indices, mut timer, mut sprite, mut texture) in &mut query {
            match (event, state.clone()) {
                (GameEvent::Idle(uid), CharacterState::Walk) => {
                    if uid != hituid {
                        continue;
                    }
                    *state = CharacterState::Idle;
                    velocity.linvel = Vec2::new(0.0, 0.0);
                    let action_name = state.to_string();
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(&action_name).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(&action_name).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);
                }
                (GameEvent::Left(uid) | GameEvent::Right(uid), CharacterState::Idle) => {
                    if uid != hituid {
                        continue;
                    }
                    *state = CharacterState::Walk;
                    match event {
                        GameEvent::Left(_) => {
                            sprite.flip_x = true;
                            velocity.linvel = Vec2::new(-200.0, 0.0);
                            *direction = Direction::Left;
                        }
                        GameEvent::Right(_) => {
                            sprite.flip_x = false;
                            velocity.linvel = Vec2::new(200.0, 0.0);
                            *direction = Direction::Right;
                        }
                        _ => panic!("Invalid event"),
                    };

                    let action_name = state.to_string();
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(&action_name).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(&action_name).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);
                }
                (GameEvent::Left(uid) | GameEvent::Right(uid), CharacterState::Walk) => {
                    if uid != hituid {
                        continue;
                    }
                    match event {
                        GameEvent::Left(_) => {
                            sprite.flip_x = true;
                            velocity.linvel = Vec2::new(-200.0, 0.0);
                            *direction = Direction::Left;
                        }
                        GameEvent::Right(_) => {
                            sprite.flip_x = false;
                            velocity.linvel = Vec2::new(200.0, 0.0);
                            *direction = Direction::Right;
                        }
                        _ => panic!("Invalid event"),
                    };
                }
                (GameEvent::Action(uid, action_name), CharacterState::Walk | CharacterState::Idle) => {
                    if uid != hituid {
                        continue;
                    }
                    *state = CharacterState::Action(action_name.clone());
                    velocity.linvel = Vec2::new(0.0, 0.0);

                    let action = characters.get(character_name.as_str()).unwrap().actions.get(action_name).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(action_name.as_str()).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);
                    if let Some(impulse) = action.internal_impulse {
                        let impulse = match *direction {
                            Direction::Left => Vec2::new(impulse.x * -1., impulse.y),
                            Direction::Right => impulse.clone(),
                        };
                        commands.entity(entity).insert(ExternalImpulse {
                            impulse,
                            torque_impulse: 0.0,
                        });
                    }
                }
                (GameEvent::Action(uid, new_action_name), CharacterState::Action(current_action_name)) => {
                    if uid != hituid {
                        continue;
                    }
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(current_action_name.as_str()).unwrap();
                    if action.frames[sprite.index].stage != ActionStage::Recovery {
                        continue;
                    }
                    // info!("action.new_action_name: {}, current_action_name: {}", new_action_name, current_action_name);
                    let action_name = if current_action_name.contains(new_action_name) { //同一技能触发连招 连招动作命名规范 attack attack2 attack3
                        // info!("action.next_action: {:?}", action.next_action);
                        if let Some(ref next_action_name) = action.next_action {
                            next_action_name.clone()
                        } else {
                            current_action_name.clone()
                        }
                    } else {
                        new_action_name.to_string()
                    };

                    *state = CharacterState::Action(action_name.clone());
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(action_name.as_str()).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(&action_name).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);
                    if let Some(impulse) = action.internal_impulse {
                        let impulse = match *direction {
                            Direction::Left => Vec2::new(impulse.x * -1., impulse.y),
                            Direction::Right => impulse.clone(),
                        };
                        commands.entity(entity).insert(ExternalImpulse {
                            impulse,
                            torque_impulse: 0.0,
                        });
                    }
                }
                (GameEvent::Stop(uid), CharacterState::Action(_) | CharacterState::Hit { .. }) => {
                    if uid != hituid {
                        continue;
                    }
                    *state = CharacterState::Idle;
                    let action_name = state.to_string();
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(&action_name).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(&action_name).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);
                    // commands.entity(entity).remove::<ExternalImpulse>(); //动作停止时清除外部冲量
                    // velocity.linvel = Vec2::new(0.0, 0.0);
                    info!("remove ExternalImpulse")
                }
                (GameEvent::Hit { uid, direction, attack_action: new_attack_action, hit_action, impulse }, _) => {
                    if uid != hituid {
                        continue;
                    }

                    if let CharacterState::Hit { ref attack_action, ref hit_action } = *state {
                        info!("current_action_name: {}, new_action_name: {}, == {}", attack_action, new_attack_action, attack_action == new_attack_action);
                        if attack_action == new_attack_action { //同一技能不连续命中
                            continue;
                        }
                    }

                    *state = CharacterState::Hit {
                        attack_action: new_attack_action.clone(),
                        hit_action: hit_action.clone(),
                    };
                    let action_name = state.to_string();
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(&action_name).unwrap();
                    let texture_atlas = characters_texture_atlas.get(character_name.as_str()).unwrap().get(&action_name).unwrap().clone();
                    *texture = texture_atlas;
                    info!("uid: {:?}", uid);
                    set_character_action(sprite, indices, timer, action);

                    if let Some(impulse) = impulse {
                        let impulse = match direction {
                            Direction::Left => Vec2::new(impulse.x * -1., impulse.y),
                            Direction::Right => impulse.clone(),
                        };
                        commands.entity(entity).insert(ExternalImpulse {
                            impulse,
                            torque_impulse: 0.0,
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

fn action(
    mut commands: Commands,
    characters: Res<Characters>,
    mut query: Query<(Entity, &CharacterState, &CharacterName, &Transform, &Direction, &TextureAtlasSprite, Option<&mut Hitbox>, Option<&mut Hurtbox>, Option<&mut Blockbox>)>,
) {
    for (entity, state, character_name, transform, direction, sprite, mut hitbox, mut hurtbox, mut blockbox) in query.iter_mut() {
        let action_name = state.to_string();
        let action = characters.get(character_name.as_str()).unwrap().actions.get(&action_name).unwrap();
        let frame = action.frames.get(sprite.index).unwrap();

        let (min_x, max_x) = match direction {
            Direction::Right => {
                (frame.hurtbox.min.x, frame.hurtbox.max.x)
            }
            Direction::Left => {
                (frame.hurtbox.max.x * -1., frame.hurtbox.min.x * -1.)
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
                    (frame_hitbox.max.x * -1., frame_hitbox.min.x * -1.)
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
                    (frame_blockbox.max.x * -1., frame_blockbox.min.x * -1.)
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
    mut hitbox_query: Query<(&Transform, &Hitbox, &UID, &Direction, &CharacterState, &CharacterName)>,
    mut hurtbox_query: Query<(&Transform, &Hurtbox, &UID)>,
    mut events: EventWriter<GameEvent>,
    characters: Res<Characters>,
) {
    for (hit_transform, hitbox, hituid, direction, character_state, character_name) in hitbox_query.iter_mut() {
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
                // info!("hit: {:?}, hurt:{:?}, x_overlap: {}, y_overlap: {}", hit, hurt, x_overlap, y_overlap );
                if x_overlap && y_overlap {
                    let action = characters.get(character_name.as_str()).unwrap().actions.get(&character_state.to_string()).unwrap();
                    info!("attack_action: {}, hit_action: {:?}, ", &character_state.to_string(), action.hit_action);
                    events.send(GameEvent::Hit {
                        uid: hurtuid.clone(),
                        direction: *direction,
                        attack_action: character_state.to_string(),
                        hit_action: action.hit_action.clone().unwrap(),
                        impulse: action.external_impulse,
                    });
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
            events.send(GameEvent::Stop(uid.clone()));
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

// fn movement(
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &Velocity)>,
// ) {
//     for (mut transform, velocity) in &mut query {
//         if velocity.0 != 0.{
//             transform.translation.x += velocity.0 * time.delta_seconds();
//         }
//     }
// }

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
        // info!("position: {}, size: {}", position, size);
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
            // info!("position: {}, size: {}", position, size);
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
            // info!("position: {}, size: {}", position, size);
            gizmos.rect_2d(
                position,
                0.,
                size,
                Color::WHITE,
            );
        }
    }
}

fn set_character_action(
    mut sprite: Mut<TextureAtlasSprite>,
    mut indices: Mut<AnimationIndices>,
    mut timer: Mut<AnimationTimer>,
    mut action: &Action,
) {
    info!("set action: {:?}", action);

    sprite.index = 0;
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