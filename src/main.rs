mod constants;
mod components;
mod levels;
mod player;


use bevy::{prelude::*, window::close_on_esc};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use float_cmp::*;
use iyes_loopless::prelude::*;
use bevy_inspector_egui::prelude::*;

use components::*;
use levels::*;
use player::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    GameOver,
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


fn read_output(
    mut commands: Commands,
    mut query: Query<(&mut EntityVelocity, &KinematicCharacterControllerOutput)>,
    coins: Query<Entity, With<Coin>>,
) {
    for (mut velocity, result) in query.iter_mut() {
        if result.effective_translation.x.approx_eq(0.0, (0.0, 2)) {
            velocity.0.x = -velocity.0.x;
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
                .with_system(camera_follow)
                .into(),
        )
        .add_system_set(ConditionSet::new().run_in_state(GameState::MainMenu).into())
        .add_system_set(ConditionSet::new().run_in_state(GameState::GameOver).into())
        .add_system(close_on_esc)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_entity::<CoinBundle>("Coin")
        .register_ldtk_entity::<HeartBundle>("Heart")
        .register_type::<EntityVelocity>()
        .run();
}
