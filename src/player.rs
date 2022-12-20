use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Bundle, LdtkIntCell, Default)]
pub struct PlayerThingsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,

    pub controller: KinematicCharacterController,
}

#[derive(Clone, Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    #[from_entity_instance]
    #[bundle]
    pub player_bundle: PlayerThingsBundle,

    #[sprite_bundle("bob_ross.png")]
    #[bundle]
    pub sprite: SpriteBundle,

    #[worldly]
    pub worldly: Worldly,

    pub player: Player,
    pub direction: PlayerDirection,
    pub velocity: EntityVelocity,
    pub controller_output: KinematicCharacterControllerOutput,
    pub coin_collector: CoinCollector,
}

impl From<EntityInstance> for PlayerThingsBundle {
    fn from(entity_instance: EntityInstance) -> PlayerThingsBundle {
        match entity_instance.identifier.as_ref() {
            "PlayerStart" => PlayerThingsBundle {
                rigid_body: RigidBody::KinematicPositionBased,
                collider: Collider::capsule_y(PLAYER_HEIGHT / 2.0 - 8.0, PLAYER_WIDTH / 2.0 - 3.0),
                controller : KinematicCharacterController::default(),
                ..default()
            },

            _ => PlayerThingsBundle::default(),
        }
    }
}

pub fn flip_player(mut query: Query<(&mut Sprite, &PlayerDirection)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = match direction {
            PlayerDirection::FaceLeft | PlayerDirection::RunLeft => false,
            PlayerDirection::FaceRight | PlayerDirection::RunRight => true,
        }
    }
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &KinematicCharacterControllerOutput,
        &mut PlayerDirection,
        &mut EntityVelocity,
    )>,
) {
    for (player, mut direction, mut velocity) in query.iter_mut() {
        if player.grounded {
            if keys.pressed(KeyCode::Z) {
                *direction = PlayerDirection::RunLeft;
            } else if keys.pressed(KeyCode::X) {
                *direction = PlayerDirection::RunRight;
            } else {
                *direction = match *direction {
                    PlayerDirection::FaceRight | PlayerDirection::RunRight => {
                        PlayerDirection::FaceRight
                    }
                    PlayerDirection::FaceLeft | PlayerDirection::RunLeft => {
                        PlayerDirection::FaceLeft
                    }
                };
            }

            if keys.just_pressed(KeyCode::Space) {
                velocity.0.y = PLAYER_JUMP;
            }
        }
    }
}

fn update_translation(t: Option<Vec2>, d: Vec2) -> Option<Vec2> {
    match t {
        Some(t) => Some(t + d),
        None => Some(d),
    }
}

pub fn apply_player_direction(
    mut query: Query<(&mut EntityVelocity, &PlayerDirection)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut velocity, direction) in query.iter_mut() {
        velocity.0.x = match direction {
            PlayerDirection::RunLeft => -PLAYER_RUN_SPEED * delta,
            PlayerDirection::RunRight => PLAYER_RUN_SPEED * delta,
            _ => 0.0,
        };
    }
}

pub fn apply_gravity(
    mut query: Query<(&mut EntityVelocity, &KinematicCharacterControllerOutput)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut velocity, result) in query.iter_mut() {
        if !result.grounded {
            velocity.0.y -= GRAVITY * delta; //gravitys
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut KinematicCharacterController, &EntityVelocity)>) {
    for (mut controller, velocity) in query.iter_mut() {
        controller.translation = update_translation(controller.translation, velocity.0);
    }
}
