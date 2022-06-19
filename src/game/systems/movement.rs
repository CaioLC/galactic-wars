use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::super::layers_util::*;
use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::components::selection::*;
use crate::game::layers_util;
use crate::math_util;

pub fn turn_to_destination(
    time: Res<Time>,
    mut query: Query<(&Transform, &Destination, &mut Velocity)>,
) {
    for (transform, destination, mut vel) in query.iter_mut() {
        if let Some(d) = destination.dest {
            let delta = (d - transform.translation).normalize();
            let target_angle = math_util::get_heading_to_point(delta);
            let cur_angle = math_util::get_heading_to_point(transform.up());
            let angle_diff = math_util::get_angle_difference(target_angle, cur_angle);
            if angle_diff.abs() > 0.005 {
                let max_angvel = 10.0_f32.min(angle_diff * time.delta_seconds() * 250.0);
                vel.angvel = Vec3::new(0.0, 0.0, max_angvel);
            }    
       };
    }
}

pub fn move_to_destination(
    time: Res<Time>,
    mut query: Query<(&mut Destination, &mut Velocity, &mut ExternalImpulse, &Avoidance, &Transform, &Movement)>,
) {
    for (mut dest, mut vel, mut impulse, avoid, transform, mov) in query.iter_mut() {
        if let Some(d) = dest.dest {
            let dist = transform.translation.distance(d);
            let accel = dist.min(mov.speed);
            // println!("{accel}");
            if dist < 1.0 {
                dest.dest = None;
                impulse.impulse = Vec3::ZERO;
                vel.angvel = Vec3::ZERO;
            } else {
                let force = (transform.up() + avoid.impulse).normalize() * accel * time.delta_seconds();
                impulse.impulse = force;   
            }
        }
    }
}

pub fn set_destination(
    ms_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
    mut query: Query<&mut Destination, With<Selected>>,
) {
    if ms_input.pressed(MouseButton::Right) {
        let ship_dest = vec2_to_vec3(mouse_pos.0, Layers::Ships);
        for mut destination in query.iter_mut() {
            match destination.dest {
                Some(d) => {
                    if d != ship_dest {
                        destination.dest = Some(ship_dest)
                    }
                }
                None => destination.dest = Some(ship_dest),
            };
        }
    }
}

pub fn damping_shift(
    mut query: Query<(&Destination, &mut Damping)>
) {
    for (destination, mut damping) in query.iter_mut() {
        match destination.dest {
            Some(_) => {
                damping.angular_damping = 0.0;
            }
            None => {
                damping.angular_damping = 20.0;
            }
        }
    }
}

pub fn collision_avoidance(
    mut ships: Query<(&mut Avoidance, &Transform)>,
    planets: Query<(&Transform, &Planet)>,
) {
    for (mut avoid, t_ship) in ships.iter_mut() {
        let ahead = layers_util::to_layer(t_ship.translation + t_ship.up() * avoid.max_see_ahead, Layers::Planets);
        for (t_planet, planet) in planets.iter() {
            // TODO: adjust code to avoid more than one planet ("find nearest threat")
            let dist = t_planet.translation.distance(ahead);
            if dist < planet.size {
                let avoid_force = ahead - t_planet.translation;
                let force = math_util::drop_z(avoid_force).normalize();
                avoid.impulse = force;
            } else if avoid.impulse != Vec3::ZERO {
                avoid.impulse = Vec3::ZERO;
            }
        }
    }
}