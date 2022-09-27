use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::Uuid,
};
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;

use super::components::{
    characteristics::*,
    players::{Ownership, PlayerDetails},
};
use super::utils::layers_util::{get_z, Layers};
use crate::{assets::materials::PlanetMaterial, selection::components::Selectable};

/// .
pub fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    font: Handle<TextMeshFont>,
    planet_type: PlanetType,
    transform: Transform,
    ownership: Option<Uuid>,
    color: Handle<PlanetMaterial>,
    no_fighters: f32,
) -> Entity {
    let radius: f32 = planet_type_to_radius(&planet_type); // from planet type return radius
    commands
        .spawn_bundle(generate_planet_mesh(radius, color, transform, meshes))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Planet {
            fighters: no_fighters,
            planet_type,
        })
        .insert(Selectable)
        .insert(Ownership(ownership))
        // ADD TEXT3D OVERLAY WITH BEVY_TEXT_MESH: https://crates.io/crates/bevy_text_mesh
        .with_children(|parent| {
            parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh {
                    text: String::from("0"),
                    style: TextMeshStyle {
                        font,
                        font_size: SizeUnit::NonStandard(90.),
                        color: Color::rgb(0.1, 0.2, 0.1),
                        mesh_quality: Quality::Custom(128),
                        ..Default::default()
                    },
                    size: TextMeshSize {
                        ..Default::default()
                    },
                    ..Default::default()
                },
                transform: Transform::from_xyz(-0.2, -0.5, get_z(Layers::Text)),
                ..Default::default()
            });
        })
        .id()
}

fn generate_planet_mesh(
    radius: f32,
    planet_material: Handle<PlanetMaterial>,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> MaterialMeshBundle<PlanetMaterial> {
    let mesh = Mesh::from(shape::Circle {
        radius,
        ..default()
    });
    MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform,
        material: planet_material,
        ..default()
    }
}

/// .
pub fn spawn_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    ship_type: ShipType,
    transform: Transform,
    set_destination: DestinationEnum,
    player_uuid: &Uuid,
    player_details: &PlayerDetails,
) -> Entity {
    let entity = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            ..Default::default()
        })
        .insert(Sensor)
        .insert(Collider::ball(0.5))
        .insert(GravityScale(0.))
        .insert(
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::TRANSLATION_LOCKED_Z,
        )
        .insert(Damping {
            linear_damping: 3.0,
            ..Default::default()
        })
        .insert(ExternalImpulse {
            ..Default::default()
        })
        .insert(Destination(set_destination))
        .insert(Selectable)
        .insert(Ownership(Some(*player_uuid)))
        .insert(Ship)
        .id();

    match ship_type {
        ShipType::Fighter => {
            commands
                .entity(entity)
                .insert_bundle(generate_ship_mesh(
                    ship_type,
                    transform,
                    meshes,
                    player_details,
                ))
                .insert(Fighter)
                .insert(Avoidance {
                    impulse: Vec3::ZERO,
                    // max_see_ahead: 8.0,
                })
                .insert(Movement { speed: 35. });
        }
        ShipType::Trade => {
            commands
                .entity(entity)
                .insert_bundle(generate_ship_mesh(
                    ship_type,
                    transform,
                    meshes,
                    player_details,
                ))
                .insert(Trader)
                .insert(TradeRoute::default())
                .insert(Avoidance {
                    impulse: Vec3::ZERO,
                    // max_see_ahead: 4.0,
                })
                .insert(Movement { speed: 12. });
        }
    }
    entity
}

fn generate_ship_mesh(
    ship_type: ShipType,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    player_details: &PlayerDetails,
) -> MaterialMeshBundle<StandardMaterial> {
    let mesh = match ship_type {
        ShipType::Fighter => ship_fighter_mesh(),
        ShipType::Trade => ship_trader_mesh(),
    };

    PbrBundle {
        mesh: meshes.add(mesh),
        transform,
        material: player_details.color.clone(),
        ..default()
    }
}

fn ship_fighter_mesh() -> Mesh {
    // points are (vec3[position], vec2[uvs])
    let points = vec![
        ([0.0, 1.0, 0.0], [1.0, 1.0]),
        ([-1.0, -1.0, 0.0], [0., 0.]),
        ([0.0, -0.5, 0.0], [0.5, 0.5]),
        ([1.0, -1.0, 0.0], [0., 0.]),
    ];
    let mut vertices = Vec::with_capacity(points.len());
    let mut uvs = Vec::with_capacity(points.len());
    let normals = vec![[0.0, 0.0, 1.0]; points.len()];

    for (position, uv) in points.iter() {
        vertices.push(*position);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
    mesh
}

fn ship_trader_mesh() -> Mesh {
    // points are (vec3[position], vec2[uvs])
    // CHECK: UVs are random values
    let points = vec![
        ([0.0, 2.0, 0.0], [1.0, 1.0]),
        ([-1.0, 1.0, 0.0], [0., 0.]),
        ([-1.0, -0.5, 0.0], [0., 0.]),
        ([0.0, -0.2, 0.0], [0.5, 0.5]),
        ([1.0, -0.5, 0.0], [0., 0.]),
        ([1.0, 1.0, 0.0], [0., 0.]),
    ];
    let mut vertices = Vec::with_capacity(points.len());
    let mut uvs = Vec::with_capacity(points.len());
    let normals = vec![[0.0, 0.0, 1.0]; points.len()];

    for (position, uv) in points.iter() {
        vertices.push(*position);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5])));
    mesh
}

fn bullet_mesh() -> Mesh {
    Mesh::from(shape::Capsule {
        radius: 3.,
        depth: 20.,
        ..Default::default()
    })
}

fn generate_bullet(
    player_details: &PlayerDetails,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> MaterialMeshBundle<StandardMaterial> {
    let mesh = bullet_mesh();
    PbrBundle {
        mesh: meshes.add(mesh),
        transform,
        material: player_details.color.clone(),
        ..default()
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    player_uuid: &Uuid,
    player_details: &PlayerDetails,
    transform: Transform,
) {
    commands
        .spawn_bundle(generate_bullet(player_details, transform, meshes))
        .insert(Ownership(Some(*player_uuid)))
        .insert(Bullet {
            origin: transform.translation,
            distance: 100.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.2))
        .insert(Sensor)
        .insert(Velocity {
            linvel: transform.up() * 80.,
            angvel: Vec3::ZERO,
        });
}
