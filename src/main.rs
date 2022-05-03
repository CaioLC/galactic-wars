use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board_plugin::BoardPlugin;
use camera_plugin::CameraPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Galactic Wars".to_string(),
        width: 1200.,
        height: 900.,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(CameraPlugin)
    .add_plugin(BoardPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
