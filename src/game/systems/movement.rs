use bevy::math::vec2;
use bevy::prelude::*;

use crate::camera::MouseWorldPos;
use crate::game::components::interact::*;
use crate::math_util::*;
use super::super::layers_util::*;

pub fn turn_to_destination(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut TurnToDestinationBehaviour,
        &mut Transform,
        &Destination,
    ),
        With<Selected>>
) {
    for (mut behaviour, mut transform, destination) in query.iter_mut() {
        let mut rotation_factor = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            rotation_factor += 1.0;
        }
    
        if keyboard_input.pressed(KeyCode::Right) {
            rotation_factor -= 1.0;
        }
        if let Some(d) = destination.dest {
            let delta = d - transform.translation;
            let desired_heading = get_heading_to_point(delta);
        };
        // let (_, _, heading_z) = transform.rotation.to_euler(EulerRot::XYZ);

        let rotation_delta = Quat::from_rotation_z(rotation_factor * behaviour.rotation_speed * time.delta_seconds());
        transform.rotation *= rotation_delta;

    }
}

pub fn set_destination(
    ms_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
    mut query: Query<&mut Destination, With<Selected>>
) {
    if ms_input.pressed(MouseButton::Right) {
        let ship_dest = vec2_to_vec3(mouse_pos.0, Layers::Ships);
        for mut destination in query.iter_mut() {
            match destination.dest {
                Some(d) => {
                    if d != ship_dest {
                        destination.dest = Some(ship_dest)
                    }
                },
                None => destination.dest = Some(ship_dest)
            };
        }
        println!("{}", mouse_pos.0);
    }
}