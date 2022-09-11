#[allow(unused_imports)]
pub mod assets;
pub mod camera;
pub mod game;
pub mod math_util;
pub mod selection;
pub mod state;
pub mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

#[cfg(feature = "debug")]
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

use assets::AssetsPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use selection::SelectionPlugin;
use state::StatePlugin;
use ui::UiPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Galactic Wars".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .insert_resource(Msaa { samples: 4 })
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(CameraPlugin)
    .add_plugin(StatePlugin)
    .add_plugin(SelectionPlugin)
    .add_plugin(GamePlugin)
    .add_plugin(AssetsPlugin)
    .add_plugin(UiPlugin);

    //.add_startup_system(UI_setup)

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<game::components::interact::Destination>::new());
        .register_inspectable::<game::components::characteristics::EnRouteBehaviour>()
        .register_inspectable::<game::components::characteristics::Movement>()
        .register_inspectable::<game::components::characteristics::Avoidance>()
        .register_inspectable::<game::components::characteristics::Planet>();

    app.run();
}
