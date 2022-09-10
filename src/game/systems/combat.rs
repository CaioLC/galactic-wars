use crate::game::{self, components::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use characteristics::{Bullet, Fighter, Trader};

pub fn bullet_hit(
    // mut ships: Query<(Entity, &Transform)>,
    // other_ships: Query<(Entity, &Transform), With<Bullet>>,
    rapier_context: Res<RapierContext>,
) {
    for (e_1, e_2, intersects) in rapier_context.intersection_pairs() {
        if intersects {
            println!("{:?}, {:?}", e_1, e_2);
        }
    }
    // for (ship_e, mut avoidance, transform) in ships.iter_mut() {
    //     avoidance.impulse = Vec3::ZERO;
    //     for (col_1, col_2, intersects) in rapier_context.intersections_with(ship_e) {
    //         if intersects {
    //             if ship_e == col_1 {
    //                 let other_ship = other_ships.get(col_2);
    //                 if let Ok((_, o_transf)) = other_ship {
    //                     let dist = o_transf.translation - transform.translation;
    //                     let repel = dist.try_normalize();
    //                     if let Some(r) = repel {
    //                         avoidance.impulse = -r
    //                     }
    //                 }
    //             } else {
    //                 let other_ship = other_ships.get(col_1);
    //                 if let Ok((_, o_transf)) = other_ship {
    //                     let dist = o_transf.translation - transform.translation;
    //                     let repel = dist.try_normalize();
    //                     if let Some(r) = repel {
    //                         avoidance.impulse = -r
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //     }
    // }
}

pub fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kb_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Fighter>>,
) {
    if kb_input.just_pressed(KeyCode::F) {
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

pub fn despawn_bullet(mut commands: Commands, query: Query<(Entity, &Bullet, &Transform)>) {
    for (e, bullet, transform) in query.iter() {
        if transform.translation.distance(bullet.origin) > bullet.distance {
            commands.entity(e).despawn_recursive();
        }
    }
}
