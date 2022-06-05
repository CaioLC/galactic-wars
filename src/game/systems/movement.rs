use bevy::prelude::*;

use super::super::layers_util::*;
use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::components::selection::*;
use crate::math_util;

pub fn turn_to_destination(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Destination)>,
) {
    for (mut transform, destination) in query.iter_mut() {
        if let Some(d) = destination.dest {
            let delta = (d - transform.translation).normalize();
            let target_angle = math_util::get_heading_to_point(delta);
            let cur_angle = math_util::get_heading_to_point(transform.up());
            let angle_diff = math_util::get_angle_difference(target_angle, cur_angle);
            if angle_diff.abs() > 0.005 {
                transform.rotation *=
                    Quat::from_rotation_z(angle_diff * time.delta_seconds() * 3.0);
            }
        };
    }
}

pub fn move_to_destination(
    time: Res<Time>,
    mut query: Query<(&mut Destination, &mut Transform, &Movement)>,
) {
    for (mut dest, mut transf, mov) in query.iter_mut() {
        if let Some(d) = dest.dest {
            let dist = transf.translation.distance(d);
            if dist < 0.1 {
                dest.dest = None;
            } else {
                let vec_target = (d - transf.translation).normalize();
                transf.translation += vec_target * time.delta_seconds() * mov.speed;
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
