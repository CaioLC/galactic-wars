use bevy::{prelude::*, utils::Uuid};

#[derive(Component)]
pub struct Ownership(pub Uuid);

#[derive(Component)]
pub struct Me;

#[derive(Component)]
pub struct Neutral;

#[derive(Component)]
pub struct Enemy;

pub enum AllegianceStatus {
    Friend,
    Neutral,
    Enemy
}