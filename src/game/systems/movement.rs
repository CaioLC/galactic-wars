use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::super::layers_util::*;
use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::resources::MovingFleets;
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
        &mut ExternalImpulse,
        &Avoidance,
        &Transform,
        &Movement,
    )>,
    mut arrived_ev_writer: EventWriter<ArrivedAtDestination>,
) {
    for (mut dest, mut impulse, avoid, transform, mov) in query.iter_mut() {
        match dest.0 {
            DestinationEnum::Space(loc) => {
                let dist = transform.translation.distance(loc);
                let accel = 4.0_f32.max(dist.min(mov.speed));
                if dist < 1.0 {
                    dest.0 = DestinationEnum::None;
                    arrived_ev_writer.send(ArrivedAtDestination(loc));
                } else {
                    let force =
                        (transform.up() + avoid.impulse).normalize() * accel * time.delta_seconds();
                    impulse.impulse = force;
                }
            }
            DestinationEnum::Planet { planet: _, loc } => {
                let dist = transform.translation.distance(loc);
                let accel = 4.0_f32.max(dist.min(mov.speed));
                let force =
                    (transform.up() + avoid.impulse).normalize() * accel * time.delta_seconds();
                impulse.impulse = force;
            }
            DestinationEnum::None => {
                if avoid.impulse != Vec3::ZERO {
                    let force = avoid.impulse.normalize() * time.delta_seconds();
                    impulse.impulse = force;
                }
            }
        }
    }
}

pub fn set_destination(
    ms_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
    mut fleets_context: ResMut<MovingFleets>,
    mut query: Query<(Entity, &mut Destination), With<Selected>>,
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
                for (_, mut destination) in query.iter_mut() {
                    destination.0 = DestinationEnum::Planet {
                        planet: e,
                        loc: ship_dest,
                    };
                }
            }
            None => {
                let mut moving_fleet = Vec::new();
                for (e, mut destination) in query.iter_mut() {
                    destination.0 = DestinationEnum::Space(ship_dest);
                    moving_fleet.push(e);
                }
                fleets_context.0.insert(ship_dest.to_string(), moving_fleet);
            }
        }
    }
}

pub fn remove_destination(
    mut fleets_context: ResMut<MovingFleets>,
    mut query: Query<(Entity, &mut Destination)>,
    mut arrived_ev_reader: EventReader<ArrivedAtDestination>,
) {
    for arrived_msg in arrived_ev_reader.iter() {
        let key = arrived_msg.0.to_string();
        let fleet = fleets_context.0.remove(&key);
        if let Some(fleet_entities) = fleet {
            for e in fleet_entities {
                let dest_res = query.get_component_mut::<Destination>(e);
                if let Ok(mut dest) = dest_res {
                    match dest.0 {
                        DestinationEnum::Space(loc) => {
                            if loc == arrived_msg.0 {
                                dest.0 = DestinationEnum::None;
                            }
                        }
                        _ => {}
                    }
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
    mut ships: Query<(Entity, &mut Avoidance, &Transform)>,
    other_ships: Query<(Entity, &Transform), With<Ship>>,
    rapier_context: Res<RapierContext>,
) {
    for (ship_e, mut avoidance, transform) in ships.iter_mut() {
        avoidance.impulse = Vec3::ZERO;
        for (col_1, col_2, intersects) in rapier_context.intersections_with(ship_e) {
            if intersects {
                if ship_e == col_1 {
                    let other_ship = other_ships.get(col_2);
                    if let Ok((_, o_transf)) = other_ship {
                        let dist = o_transf.translation - transform.translation;
                        let repel = dist.try_normalize();
                        if let Some(r) = repel {
                            avoidance.impulse = -r
                        }
                    }
                } else {
                    let other_ship = other_ships.get(col_1);
                    if let Ok((_, o_transf)) = other_ship {
                        let dist = o_transf.translation - transform.translation;
                        let repel = dist.try_normalize();
                        if let Some(r) = repel {
                            avoidance.impulse = -r
                        }
                    }
                }
                break;
            }
        }
    }
}
