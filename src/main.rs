pub mod camera;
pub mod game;
pub mod math_util;
pub mod state;
pub mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

pub use camera::CameraPlugin;
pub use game::GamePlugin;
pub use state::StatePlugin;
pub use ui::UiPlugin;

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
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(CameraPlugin)
    .add_plugin(StatePlugin)
    .add_plugin(UiPlugin)
    .add_plugin(GamePlugin);
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

// #[derive(Component)]
// struct FpsText;

// fn UI_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
// commands.spawn_bundle(UiCameraBundle::default());
// commands.spawn_bundle(TextBundle {
// style: Style {
// align_self: AlignSelf::FlexEnd,
// position_type: PositionType::Absolute,
// ..Default::default()
// },
// text: Text::with_section(
// "1",
// TextStyle {
// font: asset_server.load("fonts/ShareTechMono.ttf"),
// font_size: 20.,
// color: Color::WHITE,
// },
// TextAlignment {
// horizontal: HorizontalAlign::Center,
// ..Default::default()
// },
// ),
// ..Default::default()
// });
// }
// fn update_text_mesh(
// time: Res<Time>,
// mut text_meshes: Query<&mut TextMesh, With<EngineTime>>,
// mut timer: ResMut<UpdateTimer>,
// ) {
// if timer.timer.tick(time.delta()).just_finished() {
// for mut text_mesh in text_meshes.iter_mut() {
// let updated_text = String::from(format!("Time = {:.3}", time.seconds_since_startup()));

// if text_mesh.text != updated_text {
// text_mesh.text = updated_text;
// }
// }
// }
// }
