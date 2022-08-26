use std::f32::consts::PI;

use crate::camera::MouseWorldPos;
use crate::game::components::characteristics::*;
use crate::game::components::players::Ownership;
use crate::game::layers_util::{vec2_to_vec3, Layers};
use crate::game::resources::{MovingFleets, PlayersRes};
use crate::game::{self, layers_util, resources};
use crate::selection::components::Selected;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;

pub fn count_fighters_deployed(
    query: Query<&Fighter>,
    mut res: ResMut<resources::FightersDeployed>,
) {
    let deployed_fighters = query.iter().count() as u32;
    res.0 = deployed_fighters;
    res.set_changed();
}

pub fn count_fighters_stored(query: Query<&Planet>, mut res: ResMut<resources::FightersStored>) {
    let mut stored_fighters = 0.;
    for p in query.iter() {
        stored_fighters += p.fighters;
    }
    res.0 = stored_fighters as u32;
    res.set_changed();
}

pub fn count_traders(query: Query<&Trader>, mut res: ResMut<resources::TotalTraders>) {
    res.0 = query.iter().count() as u32;
}

pub fn production_tick(mut query: Query<(&mut Planet, &Ownership)>) {
    for (mut planet, owner) in query.iter_mut() {
        if let Some(_) = owner.0 {
            planet.fighters += 1.;
        }
    }
}

pub fn fighter_enters_planet(
    mut query: Query<(Entity, &mut Planet, &Ownership)>,
    query_ships: Query<(Entity, &Fighter, &Destination, &Ownership)>,
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_writer: EventWriter<TakeOwnership>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(planet, ship, _) => {
                if let Ok((_, _, dest, ship_owner)) = query_ships.get(*ship) {
                    if let DestinationEnum::Planet { planet: p, loc: _ } = dest.0 {
                        if *planet == p {
                            commands.entity(*ship).despawn_recursive();
                            if let Ok((entity, mut planet, planet_owner)) = query.get_mut(*planet) {
                                // IF planet and owner matches, "store" ship in planet
                                if ship_owner.0 == planet_owner.0 {
                                    planet.fighters += 1.;
                                }
                                // ELSE, it is neutral or enemy planet. Take ownership if planet has zero ships or destroy local ships
                                else {
                                    if planet.fighters == 0. {
                                        ev_writer.send(TakeOwnership {
                                            entity,
                                            owner: ship_owner.0.unwrap(),
                                        });
                                    } else {
                                        planet.fighters -= 1.;
                                    }
                                }
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
    mut set: ParamSet<(
        Query<(Entity, &Planet, &Transform)>,
        Query<(&mut Planet, &Ownership, &GlobalTransform), With<Selected>>,
    )>,
    mut fleets_context: ResMut<MovingFleets>,
    players: Res<PlayersRes>,
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPos>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        let mut find_destination = None;
        let planet_dest = vec2_to_vec3(mouse_pos.0, Layers::Planets);
        let ship_dest = vec2_to_vec3(mouse_pos.0, Layers::Ships);
        for (e, planet, transform) in set.p0().iter() {
            if planet_dest.distance(transform.translation) < planet.size {
                find_destination = Some(e);
                break;
            }
        }
        let dest = match find_destination {
            Some(e) => DestinationEnum::Planet {
                planet: e,
                loc: ship_dest,
            },
            None => DestinationEnum::Space(ship_dest),
        };

        let mut moving_fleet = Vec::new();
        for (mut planet, owner, transform) in set.p1().iter_mut() {
            if let Some(p_uuid) = owner.0 {
                let player_details = players.0.get(&p_uuid).unwrap();
                for i in 0..planet.fighters as i32 {
                    let ship_pos =
                        compute_ship_spawn_position(i, transform.translation, planet.size);
                    let entity = game::spawn_ship(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        ShipType::Fighter,
                        ship_pos,
                        dest.clone(),
                        &p_uuid,
                        player_details,
                    );
                    match dest {
                        DestinationEnum::Space(_) => {
                            moving_fleet.push(entity);
                        }
                        _ => {}
                    }
                }
                planet.fighters = 0.0;
            }
        }
        if !moving_fleet.is_empty() {
            fleets_context.0.insert(ship_dest.to_string(), moving_fleet);
        }
    }
}

fn compute_ship_spawn_position(i: i32, translation: Vec3, size: f32) -> Transform {
    let angle_pos = i as f32 * (PI / 8. + i as f32);
    let x = (size + 2.0) * angle_pos.cos() + translation.x;

    let y = (size + 2.0) * angle_pos.sin() + translation.y;
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

pub fn take_planet_ownership(
    mut ev_reader: EventReader<TakeOwnership>,
    players: Res<PlayersRes>,
    mut query: Query<(With<Planet>, &mut Ownership, &mut Handle<StandardMaterial>)>,
) {
    for event in ev_reader.iter() {
        let player_details = players.0.get(&event.owner).unwrap();
        if let Ok((_, mut p_owner, mut p_material)) = query.get_mut(event.entity) {
            p_owner.0 = Some(event.owner);
            *p_material = player_details.color.clone();
        }
    }
}
