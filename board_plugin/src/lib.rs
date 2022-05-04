use bevy::{
    gltf::Gltf,
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues},
        render_resource::VertexFormat,
    },
};
mod components;
mod systems;

pub use components::*;
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FighterTimer(Timer::from_seconds(2.0, true)))
            .insert_resource(TraderTimer(Timer::from_seconds(5.0, true)))
            .add_startup_system(setup)
            .add_system(systems::production::produce_fighters);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(generate_planet(
        1.3,
        Color::BISQUE,
        Transform::from_xyz(10., 2., 0.),
        &mut meshes,
        &mut materials,
    ));

    commands
        .spawn_bundle(generate_planet(
            1.45,
            Color::hex("ffd891").unwrap(),
            Transform::default(),
            &mut meshes,
            &mut materials,
        ))
        .insert(Planet::default());
        // ADD TEXT3D OVERLAY WITH BEVY_TEXT_MESH: https://crates.io/crates/bevy_text_mesh
        // .with_children(|parent| {
            // parent.spawn_bundle(Text2dBundle {
                // text: Text::with_section(
                    // "0".to_string(),
                    // TextStyle {
                        // font: asset_server.load("fonts/ShareTechMono.ttf"),
                        // font_size: 25.,
                        // color: Color::BLACK,
                    // },
                    // TextAlignment {
                        // vertical: VerticalAlign::Center,
                        // horizontal: HorizontalAlign::Center,
                    // },
                // ),
                // transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                // ..Default::default()
            // });
        // });

    commands
        .spawn_bundle(generate_ship(
            ShipType::Fighter,
            Transform::from_xyz(2., 2., 0.),
            &mut meshes,
            &mut materials,
        ))
        .insert(Ship { movement_speed: 2. });

    commands
        .spawn_bundle(generate_ship(
            ShipType::Trade,
            Transform::from_xyz(-2., -2., 0.),
            &mut meshes,
            &mut materials,
        ))
        .insert(Ship { movement_speed: 2. });
    // .spawn_bundle(generate_ship(ship_handle, &mut meshes, &mut materials))
    // .insert(Ship { movement_speed: 1., life: 2 });
}

fn generate_planet(
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

fn generate_ship(
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
        ([0.0, 2.0, 0.0], [1.0, 1.0]),
        ([-1.0, 0.0, 0.0], [0., 0.]),
        ([0.0, 0.5, 0.0], [0.5, 0.5]),
        ([1.0, 0.0, 0.0], [0., 0.]),
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
