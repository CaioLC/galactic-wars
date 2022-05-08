mod components;
mod systems;

use bevy::{ prelude::*, };

pub use components::*;
pub use systems::*;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseWorldPos::default())
            .add_startup_system(camera_setup)
            .add_system(camera_system)
            .add_system(mouse_to_world_pos);
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
        .insert(MainCamera);
}
