use crate::game::{
    self,
    components::*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use characteristics::{Bullet, Trader, Fighter};

/* Cast a ray inside of a system. */
pub fn cast_ray(
    rapier_context: Res<RapierContext>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Bullet>>,
) {
    for transform in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            println!("Fire!");
            let ray_pos = transform.translation;
            let ray_dir = transform.up();
            let max_toi = 0.5;
            let solid = true;
            let groups = InteractionGroups::all();
            let filter = None;
            if let Some((entity, toi)) =
                rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, groups, filter)
            {
                // The first collider hit has the entity `entity` and it hit after
                // the ray travelled a distance equal to `ray_dir * toi`.
                let hit_point = ray_pos + ray_dir * toi;
                println!("Entity {:?} hit at point {}", entity, hit_point);
            }
        }
    }

    // if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
    // ray_pos, ray_dir, max_toi, solid, groups, filter
    // ) {
    // // This is similar to `QueryPipeline::cast_ray` illustrated above except
    // // that it also returns the normal of the collider shape at the hit point.
    // let hit_point = intersection.point;
    // let hit_normal = intersection.normal;
    // println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
    // }

    // rapier_context.intersections_with_ray(
    // ray_pos, ray_dir, max_toi, solid, groups, filter,
    // |entity, intersection| {
    // // Callback called on each collider hit by the ray.
    // let hit_point = intersection.point;
    // let hit_normal = intersection.normal;
    // println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
    // true // Return `false` instead if we want to stop searching for other hits.
    // });
}

pub fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kb_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Fighter>>,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        for transform in query.iter() {
            let mut bullet_transform = transform.clone();
            bullet_transform.translation += transform.up() * 1.5;
            game::spawn_bullet(
                &mut commands,
                &mut meshes,
                &mut materials,
                bullet_transform.with_scale(Vec3::new(0.02, 0.04, 1.)),
                Color::RED,
            );
        }
    }
}
 
pub fn despawn_bullet(
    mut commands: Commands,
    query: Query<(Entity, &Bullet, &Transform)>
) {
    for (e, bullet, transform) in query.iter() {
        if transform.translation.distance(bullet.origin) > bullet.distance {
            commands.entity(e).despawn_recursive();
        }
    }
}