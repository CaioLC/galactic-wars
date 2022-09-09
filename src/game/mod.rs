pub mod components;
pub mod layers_util;
pub mod resources;
mod systems;

use std::time::Duration;

use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    utils::{HashMap, Uuid},
};
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;
use iyes_loopless::prelude::*;

use crate::player_mngmt::{
    components::{Ownership, PlayerDetails},
    resources::RegisteredPlayers,
};
use crate::selection::components::Selectable;
use crate::state::GameState;

use components::{characteristics::*, config::*};
use resources::*;
use systems::*;

use self::layers_util::{get_z, Layers};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::FightersDeployed(0))
            .insert_resource(resources::FightersStored(0))
            .insert_resource(resources::TotalTraders(0))
            .insert_resource(resources::TotalDreadnoughts(0))
            .insert_resource(resources::TotalPlanets(0))
            .insert_resource(resources::MovingFleets(HashMap::new()))
            .insert_resource(resources::GameStatus(GameStatusEnum::Uninitialized))
            .add_event::<TakeOwnership>()
            .add_event::<ArrivedAtDestination>()
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
                    .with_system(production::take_planet_ownership)
                    .with_system(movement::turn_to_destination)
                    .with_system(movement::move_to_destination)
                    .with_system(movement::set_destination)
                    .with_system(movement::remove_destination)
                    .with_system(movement::damping_shift)
                    .with_system(movement::collision_avoidance)
                    .with_system(combat::cast_ray)
                    .with_system(combat::fire_bullet)
                    .with_system(combat::despawn_bullet)
                    // this should be moved to a system set that runs at the end of frame
                    .with_system(production::count_fighters_deployed)
                    .with_system(production::count_fighters_stored)
                    .with_system(production::count_traders)
                    .into(),
            );

        #[cfg(feature = "debug")]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    board_params: Res<InitGameSetup>,
    players: Res<RegisteredPlayers>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Set Player starting planets
    for (pk, pd) in players.0.iter() {
        let transf = random_pos();
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            asset_server.load("fonts/ShareTechMono.ttf"),
            // Planet config
            5.0,
            transf,
            Some(*pk),
            pd.color.clone(),
            0.,
        );
    }

    // Set other planets
    let non_player_color = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        unlit: true,
        ..Default::default()
    });
    for _ in 0..board_params.no_of_planets {
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            asset_server.load("fonts/ShareTechMono.ttf"),
            // Planet config
            3.0,
            random_pos(),
            None,
            non_player_color.clone(),
            20.,
        );
    }
}

fn random_pos() -> Transform {
    let z = layers_util::get_z(Layers::Planets);
    let x = (rand::random::<f32>() - 0.5) * 80.;
    let y = (rand::random::<f32>() - 0.5) * 80.;
    dbg!(x, y, z);
    Transform::from_xyz(x, y, z)
}

fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    font: Handle<TextMeshFont>,
    radius: f32,
    transform: Transform,
    ownership: Option<Uuid>,
    color: Handle<StandardMaterial>,
    no_fighters: f32,
) -> Entity {
    commands
        .spawn_bundle(generate_planet_mesh(
            radius, color, transform, meshes, materials,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(radius))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Planet {
            fighters: no_fighters,
            size: radius,
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
        })
        .id()
}

fn generate_planet_mesh(
    radius: f32,
    color: Handle<StandardMaterial>,
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
        material: color,
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
                    max_see_ahead: 8.0,
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
                .insert(Avoidance {
                    impulse: Vec3::ZERO,
                    max_see_ahead: 4.0,
                })
                .insert(Movement { speed: 12. });
        }
    }
    entity
}

pub fn generate_ship_mesh(
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
