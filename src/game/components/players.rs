use bevy::{prelude::*, utils::Uuid};

#[derive(Component)]
pub struct Ownership(pub Option<Uuid>);

pub struct PlayerDetails {
    pub name: String,
    pub color: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Me;

#[derive(Component)]
pub struct Neutral;

#[derive(Component)]
pub struct Enemy;

pub enum AllegianceStatus {
    Friend,
    Neutral,
    Enemy,
}
