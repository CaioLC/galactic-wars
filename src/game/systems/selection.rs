use bevy::prelude::*;

use crate::camera::MouseWorldPos;
use crate::game::components::selection::*;
use crate::game::layers_util;

pub fn click_select(
    ms_input: Res<Input<MouseButton>>,
    ms_pos: Res<MouseWorldPos>,
    query: Query<(Entity, &Transform), With<Selectable>>,
    mut ev_select_nearest: EventWriter<SelectNearest>,
) {
    if ms_input.just_pressed(MouseButton::Left) {
        let mut min_dist: f32 = 10.0;
        let mut min_entity: Option<Entity> = None;
        for (e, transform) in query.iter() {
            let mouse_pos_3d = layers_util::vec2_to_vec3(ms_pos.0, layers_util::Layers::Planets);
            let obj_dist = mouse_pos_3d.distance_squared(transform.translation);
            if obj_dist <= min_dist {
                min_dist = obj_dist;
                min_entity = Some(e);
            }
        }
        if let Some(entity) = min_entity {
            ev_select_nearest.send(SelectNearest(entity));
        }
    }
}

pub fn update_selected(
    mut commands: Commands,
    mut ev_select_nearest: EventReader<SelectNearest>,
    query: Query<Entity, With<Selected>>,
) {
    for ev in ev_select_nearest.iter() {
        for s_entity in query.iter() {
            commands.entity(s_entity).remove::<Selected>();
        }
        commands.entity(ev.0).insert(Selected);
        println!("Entity {:?} is now selected", ev.0);
    }

}
pub fn box_select() {}

