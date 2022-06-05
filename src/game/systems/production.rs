use crate::camera::MouseWorldPos;
use crate::game::{self, layers_util};
use crate::game::{components::characteristics::*, Selected};
use bevy::prelude::*;
use bevy_text_mesh::prelude::*;

pub fn produce_fighters(
    time: Res<Time>,
    mut timer: ResMut<FighterTimer>,
    mut query: Query<&mut Planet>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut planet in query.iter_mut() {
            planet.fighters += 1.;
        }
    };
}

pub fn deploy_fighters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Planet, &Transform), With<Selected>>,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        for (mut planet, transform) in query.iter_mut() {
            let dest = layers_util::vec2_to_vec3(mouse_pos.0, layers_util::Layers::Ships);
            for _ in 0..planet.fighters as i32 {
                game::spawn_ship(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    ShipType::Fighter,
                    transform.clone(),
                    Some(dest)
                )
            }
            planet.fighters = 0.0;
        }
    }
}
pub fn update_count_mesh(mut q_child: Query<(&Parent, &mut TextMesh)>, q_parent: Query<&Planet>) {
    // TODO: CHECK IF QUERYING ALL TEXTMESHES IS OK OR WE NEED TO ADD A COMPONENT TO LIMIT FILTER.
    for (parent, mut text_mesh) in q_child.iter_mut() {
        let parent_planet = q_parent.get(parent.0);
        if let Ok(planet) = parent_planet {
            let updated_text = format!("{}", planet.fighters);
            if text_mesh.text != updated_text {
                text_mesh.text = updated_text;
            }
        }
    }
}
