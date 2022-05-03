use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
struct MapCamera;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup)
            .add_system(camera_system);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
            orthographic_projection: OrthographicProjection {
                scale: 0.05,
                ..default()
            },
            ..OrthographicCameraBundle::new_3d()
        })
        .insert(MapCamera);
}

fn camera_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ms_wheel_rdr: EventReader<MouseWheel>,
    mut ev_motion: EventReader<MouseMotion>,
    ms_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<MapCamera>>,
) {
    let mut transf = query.single_mut();
    let mut direction = Vec3::ZERO;
    let scale: f32 = transf.scale.x;
    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Z) {
        let scale = scale + 0.1;
        transf.scale = Vec3::splat(scale);
    }

    if keyboard_input.pressed(KeyCode::X) {
        let scale = scale - 0.1;
        transf.scale = Vec3::splat(scale);
    }
    for e in ms_wheel_rdr.iter() {
        let scale = scale - e.y * 0.1;
        transf.scale = Vec3::splat(scale);
    }
    if transf.scale.x < 1.0 {
        transf.scale = Vec3::splat(1.)
    }
    if ms_input.pressed(MouseButton::Left) {
        println!("left button pressed")
        // transf.translation.z += 1. * e.y;
    }
    transf.translation += time.delta_seconds() * direction * 100.;
}
