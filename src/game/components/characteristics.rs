use bevy::{prelude::*, utils::Uuid};

// EVENTS

// COMPONENTS
#[derive(Component)]
pub struct Fighter;

#[derive(Component)]
pub struct Trader;

#[derive(Component)]
pub struct Bullet {
    pub origin: Vec3,
    pub distance: f32,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Avoidance {
    pub impulse: Vec3,
    pub max_see_ahead: f32,
}

#[derive(Component)]
pub struct Destination(pub DestinationEnum);

pub enum DestinationEnum {
    None,
    Space(Vec3),
    Planet { planet: Entity, loc: Vec3 },
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct EnRouteBehaviour {
    pub point_a: Vec3,
    pub point_b: Vec3,
    pub en_route_to: Vec3,
}
pub enum ShipType {
    Trade,
    Fighter,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Planet {
    pub fighters: f32,
    pub size: f32,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            fighters: 0.,
            size: 1.,
        }
    }
}

#[derive(Component)]
pub struct FighterProducer;
#[derive(Component)]
pub struct TraderProducer;

pub struct TakeOwnership {
    pub entity: Entity,
    pub owner: Uuid,
}
