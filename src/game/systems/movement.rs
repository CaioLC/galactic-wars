use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::super::layers_util::*;
use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::components::selection::*;
use crate::math_util;

pub fn turn_to_destination(
    time: Res<Time>,
    mut query: Query<(&Transform, &Destination, &mut Velocity), With<Selected>>,
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
    mut query: Query<(&mut Destination, &mut Velocity,&Transform, &Movement)>,
) {
    for (mut dest, mut vel, transform, mov) in query.iter_mut() {
        if let Some(d) = dest.dest {
            let dist = transform.translation.distance(d);
            if dist < 0.5 {
                dest.dest = None;
                vel.linvel = Vec3::ZERO;
                vel.angvel = Vec3::ZERO;
            } else {
                let max_speed = 10.0_f32.min(time.delta_seconds() * mov.speed * 40. * dist);
                vel.linvel = transform.up() * max_speed;
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
