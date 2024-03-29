use bevy::{prelude::*, utils::Uuid};
use bevy_rapier3d::prelude::*;

use crate::assets::materials::PlanetMaterial;

#[derive(Component)]
pub struct Ownership(pub Option<Uuid>);

pub struct PlayerDetails {
    pub name: String,
    pub color: Handle<StandardMaterial>,
    pub new_color: Handle<PlanetMaterial>,
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

pub enum AllegianceCollisionGroups {
    Friend = 0b1101,
    Neutral = 0b0010,
    Enemy = 0b0011,
}

pub const INTERACT: InteractionGroups = InteractionGroups::new(1, 1);
