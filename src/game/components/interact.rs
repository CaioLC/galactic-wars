use bevy::prelude::*;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Movement{
    pub speed: f32,
}

#[derive(Component)]
pub struct AttackBehaviour {
    pub accuracy: f32,
    pub range: f32,
}

#[derive(Component)]
pub struct TurnToDestinationBehaviour {
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct Destination{
    pub dest: Option<Vec3>
}

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
