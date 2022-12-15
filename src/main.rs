mod levels;

use bevy::{prelude::*, window::close_on_esc};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use levels::*;

const PLAYER_WIDTH: f32 = 22.0;
const PLAYER_HEIGHT: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.25,
            ..default()
        },
        ..default()
    });

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });

    commands.insert_resource(LevelSelection::Index(0))
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = "pixelplatformer/Characters/character_0000.png";

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(sprite),
            transform: Transform {
                translation: Vec3::new(0.0, 40., 4.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::ZERO))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0))
        .insert(KinematicCharacterController {offset: CharacterLength::Absolute(0.01), ..default() })
        .insert(KinematicCharacterControllerOutput::default());
}

fn update_translation(t: Option<Vec2>, d: Vec2) -> Option<Vec2> {
    match t {
        Some(t) => Some(t + d),
        None => Some(d),
    }
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut Velocity)>,
) {
    let (mut controller,  player, mut velocity) = query.single_mut();
    if player.grounded {
        velocity.0 = Vec2::ZERO;

        if keys.pressed(KeyCode::Z) {
            velocity.0.x = -1.0;
        } else if keys.pressed(KeyCode::X) {
            velocity.0.x = 1.0;
        }

        if keys.pressed(KeyCode::Space) {
            velocity.0.y = 2.0;
        }
    } else {
        velocity.0.y -= 0.05; //gravity
    }

    controller.translation = update_translation(controller.translation, velocity.0)
}

fn camera_follow(
    player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player.single();
    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(
        player.translation.x,
        player.translation.y + 50.0,
        camera.translation.z,
    );
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1270.0,
                        height: 720.0,
                        title: String::from("Bob Ross Adventure"),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_loopless_state(GameState::InGame)
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(startup)
        .add_startup_system(player_setup)
        .add_system(player_input)
        .add_system(camera_follow)
        .add_system(close_on_esc)
        .add_system(spawn_wall_collision)
        .register_ldtk_int_cell::<WallBundle>(1)
        .run();
}
