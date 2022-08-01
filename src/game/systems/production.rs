use std::f32::consts::PI;
use std::iter::Enumerate;

use crate::camera::MouseWorldPos;
use crate::game::layers_util::Layers;
use crate::game::{self, layers_util};
use crate::game::{components::characteristics::*};
use crate::selection::components::Selected;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;

pub fn produce_fighters(
    time: Res<Time>,
    mut timer: ResMut<FighterTimer>,
    mut query: Query<(Entity, &mut Planet)>,
    query_ships: Query<(Entity, &Fighter, &Destination)>,
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (_, mut planet) in query.iter_mut() {
            planet.fighters += 1.;
        }
    };
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(planet, ship, _) => {
                if let Ok((_, _, dest)) = query_ships.get(*ship) {
                    if let DestinationEnum::Planet { planet: p, loc: _ } = dest.0 {
                        if *planet == p {
                            commands.entity(*ship).despawn_recursive();
                            if let Ok((_, mut planet)) = query.get_mut(*planet) {
                                planet.fighters += 1.;
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn deploy_fighters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Planet, &GlobalTransform), With<Selected>>,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        for (mut planet, transform) in query.iter_mut() {
            let dest = layers_util::vec2_to_vec3(mouse_pos.0, layers_util::Layers::Ships);
            for i in 0..planet.fighters as i32 {
                let ship_pos = compute_ship_spawn_position(i, transform.translation, planet.size);
                game::spawn_ship(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    ShipType::Fighter,
                    ship_pos,
                    DestinationEnum::Space(dest),
                )
            }
            planet.fighters = 0.0;
        }
    }
}

fn compute_ship_spawn_position(i: i32, translation: Vec3, size: f32) -> Transform {
    let angle_pos = i as f32 * PI / 80.0;
    println!("{angle_pos}");
    let x = translation.x + (size + 1.0) + angle_pos.cos();
    let y = translation.y + (size + 1.0) * angle_pos.sin();
    let z = layers_util::get_z(Layers::Ships);
    Transform::from_xyz(x, y, z)
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
