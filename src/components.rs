use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Component, Default)]
pub enum PlayerDirection {
    FaceLeft,
    #[default]
    FaceRight,
    RunLeft,
    RunRight,
}

#[derive(Component, Default, Clone)]
pub struct CoinCollector(u32);

#[derive(Component, Reflect, Inspectable, Default, Clone)]
#[reflect(Component)]
pub struct EntityVelocity(pub Vec2);
