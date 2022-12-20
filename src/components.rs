use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Clone, Component, Default, Reflect, Inspectable)]
#[reflect(Component)]
pub enum PlayerDirection {
    FaceLeft,
    #[default]
    FaceRight,
    RunLeft,
    RunRight,
}

#[derive(Component, Default, Clone, Reflect, Inspectable)]
#[reflect(Component)]
pub struct CoinCollector(pub u32);

#[derive(Component, Reflect, Inspectable, Default, Clone)]
#[reflect(Component)]
pub struct EntityVelocity(pub Vec2);


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Coin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Heart;