mod levels;

use bevy::{prelude::*, window::close_on_esc};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use float_cmp::*;
use iyes_loopless::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use levels::*;

const PLAYER_WIDTH: f32 = 22.0;
const PLAYER_HEIGHT: f32 = 24.0;
const PLAYER_RUN_SPEED: f32 = 100.0;
const PLAYER_JUMP: f32 = 3.0;
const GRAVITY: f32 = 9.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct CoinCollector(u32);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
enum PlayerDirection {
    FaceLeft,
    FaceRight,
    RunLeft,
    RunRight,
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.25,
            ..default()
        },
        ..default()
    });
}

fn level_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });

    commands.insert_resource(LevelSelection::Index(0))
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = "bob_ross.png";

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(sprite),
            transform: Transform {
                translation: Vec3::new(20.0, 40., 4.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::ZERO))
        .insert(RigidBody::KinematicPositionBased)
        .insert(CoinCollector::default())
        .insert(Collider::capsule_y(
            PLAYER_HEIGHT / 2.0 - 8.0,
            PLAYER_WIDTH / 2.0 - 3.0,
        ))
        .insert(KinematicCharacterController {  ..default() })
        .insert(KinematicCharacterControllerOutput::default())
        .insert(PlayerDirection::FaceRight);
}

fn update_translation(t: Option<Vec2>, d: Vec2) -> Option<Vec2> {
    match t {
        Some(t) => Some(t + d),
        None => Some(d),
    }
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &KinematicCharacterControllerOutput,
        &mut PlayerDirection,
        &mut Velocity,
    )>,
) {
    let (player, mut direction, mut velocity) = query.single_mut();
    if player.grounded {
        if keys.pressed(KeyCode::Z) {
            *direction = PlayerDirection::RunLeft;
        } else if keys.pressed(KeyCode::X) {
            *direction = PlayerDirection::RunRight;
        } else {
            *direction = 
                match *direction {
                    PlayerDirection::FaceRight | PlayerDirection::RunRight => PlayerDirection::FaceRight,
                    PlayerDirection::FaceLeft | PlayerDirection::RunLeft => PlayerDirection::FaceLeft,
                };
            }

        if keys.just_pressed(KeyCode::Space) {
            velocity.0.y = PLAYER_JUMP;
        }
    }
    //if keys.just_pressed(KeyCode::Space) {
    //    velocity.0.y = PLAYER_JUMP;
    //}
    //} else {
    //    velocity.0.y -= GRAVITY * delta; //gravity
    //}

    //println!("{:?}", controller.translation);
}

fn apply_player_direction(mut query: Query<(&mut Velocity, &PlayerDirection)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (mut velocity, direction) in query.iter_mut() {
        let x = match direction {
            PlayerDirection::RunLeft => -PLAYER_RUN_SPEED * delta,
            PlayerDirection::RunRight => PLAYER_RUN_SPEED * delta,
            _ => 0.0,
        };

        velocity.0.x = x;
    }
}

fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut velocity in query.iter_mut() {
        velocity.0.y -= GRAVITY * delta; //gravitys
    }
}

fn apply_velocity(mut query: Query<(&mut KinematicCharacterController, &Velocity)>) {
    for (mut controller, velocity) in query.iter_mut() {
        controller.translation = update_translation(controller.translation, velocity.0);
    }
}

fn read_output(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &KinematicCharacterControllerOutput)>,
    coins: Query<Entity, With<Coin>>,
) {
    for (velocity, result) in query.iter_mut() {
        if result.effective_translation.x.approx_eq(0.0, (0.0, 2)) {
            /*if velocity.0.x > 0.0 {
                println!("Correcting Left");
                controller.translation = Some(Vec2::new(-0.1, 0.0));
            } else if velocity.0.x < 0.0 {

                println!("Correcting Right");
                controller.translation = Some(Vec2::new(0.1, 0.0));
            }*/

            //velocity.0.x = 0.0; //= 2.0; //Stop trying to move L/R if weve it something

            //if result.effective_translation.y.approx_eq(0.0, (0.0, 5)) && velocity.0.y > 0.0 {
            //    velocity.0.y = 0.0; //Stop trying to move Up if weve it something
            //}
        }

        for collision in result.collisions.iter() {
            let coin = coins.get(collision.entity);
            if let Ok(coin) = coin {
                println!("Collected coin");
                commands.entity(coin).despawn();
            }
        }
    }
}

fn detect_collisions(
    mut _coin_collectors: Query<&mut CoinCollector>,
    _coins: Query<Entity, With<Coin>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        println!("Collision");
        match collision {
            CollisionEvent::Started(_collider_a, _collider_b, _) => {}
            _ => {}
        }
    }
}

fn camera_follow(
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(
        player.translation.x,
        player.translation.y + 60.0,
        camera.translation.z,
    );
}

fn flip_player(
    mut query: Query<(&mut Sprite, &PlayerDirection)>) {
    let (mut sprite, direction) = query.single_mut();
    sprite.flip_x = 
    match direction { 
        PlayerDirection::FaceLeft | PlayerDirection::RunLeft => false,
        PlayerDirection::FaceRight | PlayerDirection::RunRight => true,
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.0,
                        height: 1024.0,
                        title: String::from("Bob Ross Adventure"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_loopless_state(GameState::InGame)
        .add_plugin(LdtkPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(startup)
        .add_enter_system(GameState::InGame, player_setup)
        .add_enter_system(GameState::InGame, level_startup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spawn_wall_collision)
                .with_system(player_input)
                .with_system(apply_player_direction)
                .with_system(apply_gravity)
                .with_system(apply_velocity)
                .with_system(flip_player)
                .with_system(read_output)
                .with_system(detect_collisions)
                .with_system(camera_follow)
                .into(),
        )
        .add_system_set(ConditionSet::new().run_in_state(GameState::MainMenu).into())
        .add_system_set(ConditionSet::new().run_in_state(GameState::GameOver).into())
        .add_system(close_on_esc)
        .register_ldtk_int_cell::<WallBundle>(1)
        //.register_ldtk_int_cell::<CoinBundle>(2)

        .register_ldtk_entity::<CoinBundle>("Coin")
        .run();
}
