use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Selected;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Movement{
    pub speed: f32,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Destination {
    pub dest: Option<Vec3>
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
