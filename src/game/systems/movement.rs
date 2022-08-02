use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::super::layers_util::*;
use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::layers_util;
use crate::selection::components::Selected;

use crate::math_util;

pub fn turn_to_destination(
    time: Res<Time>,
    mut query: Query<(&Transform, &Destination, &mut Velocity)>,
) {
    for (transform, destination, mut vel) in query.iter_mut() {
        match destination.0 {
            DestinationEnum::Space(d) => {
                let angle_diff = turn_to_dest_math(d, transform.translation, transform.up());
                if angle_diff.abs() > 0.005 {
                    let max_angvel = 10.0_f32.min(angle_diff * time.delta_seconds() * 250.0);
                    vel.angvel = Vec3::new(0.0, 0.0, max_angvel);
                }
            }
            DestinationEnum::Planet { planet: _, loc } => {
                let angle_diff = turn_to_dest_math(loc, transform.translation, transform.up());
                if angle_diff.abs() > 0.005 {
                    let max_angvel = 10.0_f32.min(angle_diff * time.delta_seconds() * 250.0);
                    vel.angvel = Vec3::new(0.0, 0.0, max_angvel);
                }
            }
            DestinationEnum::None => {}
        }
    }
}

fn turn_to_dest_math(target: Vec3, pos: Vec3, up_pos: Vec3) -> f32 {
    let delta = (target - pos).normalize();
    let target_angle = math_util::get_heading_to_point(delta);
    let cur_angle = math_util::get_heading_to_point(up_pos);
    math_util::get_angle_difference(target_angle, cur_angle)
}

pub fn move_to_destination(
    time: Res<Time>,
    mut query: Query<(
        &mut Destination,
        &mut Velocity,
        &mut ExternalImpulse,
        &Avoidance,
        &Transform,
        &Movement,
    )>,
) {
    for (mut dest, mut vel, mut impulse, avoid, transform, mov) in query.iter_mut() {
        match dest.0 {
            DestinationEnum::Space(loc) => {
                let dist = transform.translation.distance(loc);
                let accel = 2.0_f32.max(dist.min(mov.speed));
                if dist < 1.0 {
                    dest.0 = DestinationEnum::None;
                    impulse.impulse = Vec3::ZERO;
                    vel.angvel = Vec3::ZERO;
                } else {
                    let force =
                        (transform.up() + avoid.impulse).normalize() * accel * time.delta_seconds();
                    impulse.impulse = force;
                }
            }
            DestinationEnum::Planet { planet: _, loc } => {
                let dist = transform.translation.distance(loc);
                let accel = 2.0_f32.max(dist.min(mov.speed));
                let force =
                    (transform.up() + avoid.impulse).normalize() * accel * time.delta_seconds();
                impulse.impulse = force;
            }
            DestinationEnum::None => {}
        }
    }
}

pub fn set_destination(
    ms_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
    mut query: Query<&mut Destination, With<Selected>>,
    planet_query: Query<(Entity, &Planet, &Transform)>,
) {
    if ms_input.pressed(MouseButton::Right) {
        let planet_dest = vec2_to_vec3(mouse_pos.0, Layers::Planets);
        let ship_dest = vec2_to_vec3(mouse_pos.0, Layers::Ships);
        let mut target_planet = None;
        for (e, planet, transform) in planet_query.iter() {
            if planet_dest.distance(transform.translation) < planet.size {
                target_planet = Some(e);
                break;
            }
        }
        match target_planet {
            Some(e) => {
                for mut destination in query.iter_mut() {
                    destination.0 = DestinationEnum::Planet {
                        planet: e,
                        loc: ship_dest,
                    };
                }
            }
            None => {
                for mut destination in query.iter_mut() {
                    destination.0 = DestinationEnum::Space(ship_dest);
                }
            }
        }
    }
}

pub fn damping_shift(mut query: Query<(&Destination, &mut Damping)>) {
    for (destination, mut damping) in query.iter_mut() {
        match destination.0 {
            DestinationEnum::None => {
                damping.angular_damping = 20.0;
            }
            _ => {
                damping.angular_damping = 0.0;
            }
        }
    }
}

pub fn collision_avoidance(
    mut ships: Query<(&mut Avoidance, &Transform, &Destination)>,
    planets: Query<(Entity, &Transform, &Planet)>,
) {
    for (mut avoid, t_ship, dest) in ships.iter_mut() {
        let ahead = layers_util::to_layer(
            t_ship.translation + t_ship.up() * avoid.max_see_ahead,
            Layers::Planets,
        );
        let mut ship_threat = None;
        let mut look_ahead_threat = None;

        for (e_planet_i, t_planet_i, planet_i) in planets.iter() {
            // Find nearest threat looking at ship position
            let dist_pos = t_planet_i.translation.distance(t_ship.translation); // TODO: calculate 2d distance disregarding layers system
            if dist_pos < planet_i.size + 0.5 {
                match ship_threat {
                    Some((_, dist)) => {
                        if dist_pos < dist {
                            ship_threat = Some((e_planet_i, dist_pos));
                        }
                    }
                    None => ship_threat = Some((e_planet_i, dist_pos)),
                }
                break;
            }

            // Find nearest threat looking ahead
            let dist_ahead = t_planet_i.translation.distance(ahead);
            if dist_ahead < planet_i.size {
                match look_ahead_threat {
                    Some((_, dist)) => {
                        if dist_ahead < dist {
                            look_ahead_threat = Some((e_planet_i, dist_ahead));
                        }
                    }
                    None => look_ahead_threat = Some((e_planet_i, dist_ahead)),
                }
            }
        }

        if let Some((entity, _)) = ship_threat {
            if let Ok((p_entity, p_transform, _)) = planets.get(entity) {
                avoid.impulse = calculate_impulse(
                    &dest.0,
                    p_entity,
                    p_transform.translation,
                    t_ship.translation,
                );
            }
        }
        if let Some((entity, _)) = look_ahead_threat {
            if let Ok((p_entity, p_transform, _)) = planets.get(entity) {
                avoid.impulse =
                    calculate_impulse(&dest.0, p_entity, p_transform.translation, ahead);
            }
        }
        if ship_threat == None && look_ahead_threat == None {
            avoid.impulse = Vec3::ZERO;
        }
    }
}

fn calculate_impulse(
    destination_enum: &DestinationEnum,
    p_entity: Entity,
    p_pos: Vec3,
    ship_pos: Vec3,
) -> Vec3 {
    match destination_enum {
        DestinationEnum::Space(_) => math_util::drop_z(ship_pos - p_pos).normalize(),
        DestinationEnum::Planet { planet: p, loc: _ } => {
            let mut res = Vec3::ZERO;
            if p_entity != *p {
                res = math_util::drop_z(ship_pos - p_pos).normalize();
            }
            res
        }
        DestinationEnum::None => Vec3::ZERO,
    }
}
