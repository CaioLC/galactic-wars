use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub movement_speed: f32,
}

pub enum ShipType {
    Trade,
    Fighter,
}

#[derive(Component)]
pub struct Planet {
    pub fighters: f32,
    pub traders: f32,
    pub size: f32,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            fighters: 0.,
            traders: 0.,
            size: 1.,
        }
    }
}

#[derive(Component)]
pub struct FighterProducer;
#[derive(Component)]
pub struct TraderProducer;

pub struct FighterTimer(pub Timer);

pub struct TraderTimer(pub Timer);
