use crate::game::{
    components::{characteristics::Ship, players::*, *},
    obj::spawn_bullet,
    resources::player_res::RegisteredPlayers,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use characteristics::{Bullet, Fighter};

pub fn bullet_hit(
    mut commands: Commands,
    bullets: Query<(Entity, &Ownership), With<Bullet>>,
    ships: Query<(Entity, &Ownership), With<Ship>>,
    rapier_context: Res<RapierContext>,
) {
    for (e_1, e_2, intersects) in rapier_context.intersection_pairs() {
        if intersects {
            let bullet_e1 = bullets.get(e_1);
            let bullet_e2 = bullets.get(e_2);
            let ship_e1 = ships.get(e_1);
            let ship_e2 = ships.get(e_2);
            if let Ok((b, b_owner)) = bullet_e1 {
                if let Ok((s, s_owner)) = ship_e2 {
                    if b_owner.0 != s_owner.0 {
                        commands.entity(b).despawn_recursive();
                        commands.entity(s).despawn_recursive();
                    }
                }
            }
            if let Ok((b, b_owner)) = bullet_e2 {
                if let Ok((s, s_owner)) = ship_e1 {
                    if b_owner.0 != s_owner.0 {
                        commands.entity(b).despawn_recursive();
                        commands.entity(s).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn fire_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    players: Res<RegisteredPlayers>,
    kb_input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Ownership), With<Fighter>>,
) {
    if kb_input.just_pressed(KeyCode::F) {
        for (transform, owner) in query.iter() {
            let mut bullet_transform = transform.clone();
            bullet_transform.translation += transform.up() * 1.5;
            let owner_details = players
                .0
                .get(&owner.0.expect("ship with no owner attempted fire"))
                .expect("failed to get details for ship owner");
            spawn_bullet(
                &mut commands,
                &mut meshes,
                &owner.0.unwrap(),
                &owner_details,
                bullet_transform.with_scale(Vec3::new(0.02, 0.04, 1.)),
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
