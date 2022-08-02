pub mod components;
pub mod layers_util;
mod systems;

use std::time::Duration;

use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;
use iyes_loopless::prelude::*;

use components::{characteristics::*, config::*};
use systems::*;

use crate::selection::components::Selectable;
use crate::state::GameState;

use self::layers_util::{get_z, Layers};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPlugin)
            .insert_resource(FighterTimer(Timer::from_seconds(3.0, true)))
            .insert_resource(TraderTimer(Timer::from_seconds(5.0, true)))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(TextMeshPlugin)
            .add_startup_system(setup)
            .add_stage_before(
                CoreStage::Update,
                "fighter_producer_tick",
                FixedTimestepStage::new(Duration::from_secs_f32(0.5))
                    .with_stage(SystemStage::parallel().with_system(production::production_tick)),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(production::fighter_enters_planet)
                    .with_system(production::deploy_fighters)
                    .with_system(production::update_count_mesh)
                    .with_system(movement::turn_to_destination)
                    .with_system(movement::move_to_destination)
                    .with_system(movement::set_destination)
                    .with_system(movement::damping_shift)
                    .with_system(movement::collision_avoidance)
                    .with_system(combat::cast_ray)
                    .with_system(combat::fire_bullet)
                    .with_system(combat::despawn_bullet)
                    .into(),
            );

        #[cfg(feature = "debug")]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    board_params: Res<BoardParams>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..board_params.no_of_planets {
        let transf = Transform::from_xyz(
            i as f32 * 10.,
            i as f32 * 10.,
            layers_util::get_z(Layers::Planets),
        );
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            2.0 * i as f32 + 3.0,
            transf,
            Color::GREEN,
            asset_server.load("fonts/ShareTechMono.ttf"),
        );
    }

    spawn_ship(
        &mut commands,
        &mut meshes,
        &mut materials,
        ShipType::Trade,
        Transform::from_xyz(-7., 2., layers_util::get_z(Layers::Ships)),
        DestinationEnum::None,
    );

    spawn_ship(
        &mut commands,
        &mut meshes,
        &mut materials,
        ShipType::Trade,
        Transform::from_xyz(-7., 10., layers_util::get_z(Layers::Ships)),
        DestinationEnum::None,
    );

    spawn_bullet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(17., 2., layers_util::get_z(Layers::Ships))
            .with_scale(Vec3::new(0.02, 0.04, 1.)),
        Color::RED,
    );
    spawn_bullet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(17., 42., layers_util::get_z(Layers::Ships))
            .with_scale(Vec3::new(0.02, 0.04, 1.)),
        Color::RED,
    );
}

// TODO:
fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    radius: f32,
    transform: Transform,
    color: Color,
    font: Handle<TextMeshFont>,
) {
    commands
        .spawn_bundle(generate_planet_mesh(
            radius, color, transform, meshes, materials,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Planet {
            fighters: 0.,
            size: radius,
        })
        .insert(Selectable)
        // ADD TEXT3D OVERLAY WITH BEVY_TEXT_MESH: https://crates.io/crates/bevy_text_mesh
        .with_children(|parent| {
            parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh {
                    text: String::from("0"),
                    style: TextMeshStyle {
                        font,
                        font_size: SizeUnit::NonStandard(70.),
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
        });
}

fn generate_planet_mesh(
    radius: f32,
    color: Color,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialMeshBundle<StandardMaterial> {
    let mesh = Mesh::from(shape::Icosphere {
        radius,
        subdivisions: 16,
    });
    PbrBundle {
        mesh: meshes.add(mesh),
        transform,
        material: materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        }),
        ..default()
    }
}

pub fn spawn_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_type: ShipType,
    transform: Transform,
    set_destination: DestinationEnum,
) {
    let entity = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            ..Default::default()
        })
        .insert(GravityScale(0.))
        .insert(
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::TRANSLATION_LOCKED_Z,
        )
        .insert(Damping {
            linear_damping: 5.0,
            ..Default::default()
        })
        .insert(ExternalImpulse {
            ..Default::default()
        })
        .insert(Destination(set_destination))
        .insert(Selectable)
        .id();

    match ship_type {
        ShipType::Fighter => {
            commands
                .entity(entity)
                .insert_bundle(generate_ship_mesh(ship_type, transform, meshes, materials))
                .insert(Fighter)
                .insert(Collider::ball(0.5))
                .insert(Avoidance {
                    impulse: Vec3::ZERO,
                    max_see_ahead: 8.0,
                })
                .insert(Movement { speed: 35. });
        }
        ShipType::Trade => {
            commands
                .entity(entity)
                .insert_bundle(generate_ship_mesh(ship_type, transform, meshes, materials))
                .insert(Trader)
                .insert(Collider::ball(0.5))
                .insert(Avoidance {
                    impulse: Vec3::ZERO,
                    max_see_ahead: 4.0,
                })
                .insert(Movement { speed: 12. });
        }
    }
}

pub fn generate_ship_mesh(
    ship_type: ShipType,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialMeshBundle<StandardMaterial> {
    let mesh = match ship_type {
        ShipType::Fighter => ship_fighter_mesh(),
        ShipType::Trade => ship_trader_mesh(),
    };

    PbrBundle {
        mesh: meshes.add(mesh),
        transform,
        material: materials.add(StandardMaterial {
            base_color: Color::BLUE,
            unlit: true,
            ..default()
        }),
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
    color: Color,
    transform: Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialMeshBundle<StandardMaterial> {
    let mesh = bullet_mesh();
    PbrBundle {
        mesh: meshes.add(mesh),
        transform,
        material: materials.add(StandardMaterial {
            base_color: color,
            unlit: true,
            ..default()
        }),
        ..default()
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
    color: Color,
) {
    commands
        .spawn_bundle(generate_bullet(color, transform, meshes, materials))
        .insert(Bullet {
            origin: transform.translation,
            distance: 50.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: transform.up() * 40.,
            angvel: Vec3::ZERO,
        });
    // .insert(Collider::ball(radius))
}
