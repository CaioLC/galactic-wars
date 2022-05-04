use std::time::Duration;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_text_mesh::prelude::*;
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
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(TextMeshPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(BoardPlugin)
    //.add_startup_system(UI_setup)
    .add_startup_system(setup_text_mesh)
    .add_system(update_text_mesh);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

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

fn setup_text_mesh(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/ShareTechMono.ttf");

    commands.spawn_bundle(TextMeshBundle {
        text_mesh: TextMesh {
            text: String::from("Time since startup"),
            style: TextMeshStyle {
                font: font.clone(),
                font_size: SizeUnit::NonStandard(9.),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Default::default()
            },
            size: TextMeshSize {
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-1., 1.75, 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("0"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(36.),
                    color: Color::rgb(0.0, 1.0, 0.0),
                    mesh_quality: Quality::Custom(128),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-1., 1.3, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EngineTime);

    commands.insert_resource(UpdateTimer {
        timer: Timer::new(Duration::from_millis(100), true),
    });
}

struct UpdateTimer {
    timer: Timer,
}

#[derive(Component)]
struct EngineTime;

fn update_text_mesh(
    time: Res<Time>,
    mut text_meshes: Query<&mut TextMesh, With<EngineTime>>,
    mut timer: ResMut<UpdateTimer>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        for mut text_mesh in text_meshes.iter_mut() {
            let updated_text = String::from(format!("Time = {:.3}", time.seconds_since_startup()));

            if text_mesh.text != updated_text {
                text_mesh.text = updated_text;
            }
        }
    }
}