use bevy::{prelude::*, utils::Uuid};
use rand::{
    distributions::{Standard, WeightedIndex},
    prelude::Distribution,
};

// EVENTS
pub struct TakeOwnership {
    pub entity: Entity,
    pub owner: Uuid,
}

pub struct ArrivedAtDestination(pub Vec3);
// COMPONENTS

#[derive(Component)]
pub struct Ship;

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

#[derive(Clone)]
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
    pub planet_type: PlanetType,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub enum PlanetType {
    Outpost,
    Watch,
    Base,
    Colony,
    Capital,
}

impl Distribution<PlanetType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PlanetType {
        match rng.gen_range(0..=9) {
            0 => PlanetType::Outpost,
            1 => PlanetType::Outpost,
            2 => PlanetType::Outpost,
            3 => PlanetType::Watch,
            4 => PlanetType::Watch,
            5 => PlanetType::Base,
            6 => PlanetType::Base,
            7 => PlanetType::Colony,
            8 => PlanetType::Colony,
            9 => PlanetType::Capital,
            _ => panic!("random generator exceede values between 0-9 for PlanetType generation"),
        }
    }
}

pub fn planet_type_to_radius(pt: &PlanetType) -> f32 {
    match pt {
        PlanetType::Outpost => 2.,
        PlanetType::Watch => 3.,
        PlanetType::Base => 5.,
        PlanetType::Colony => 8.,
        PlanetType::Capital => 12.,
    }
}

#[derive(Component)]
pub struct FighterProducer;
#[derive(Component)]
pub struct TraderProducer;
