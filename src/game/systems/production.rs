use crate::game::components::interact::*;
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
            // println!("fighter produced. Total: {:?}", planet.fighters)
        }
    };
}

pub fn update_count_mesh(
    mut q_child: Query<(&Parent, &mut TextMesh)>,
    q_parent: Query<&Planet>
) {
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