use bevy::prelude::{App, AssetServer, Commands, Handle, Plugin, Res, ResMut, State, SystemSet};
use bevy::window::Windows;
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::bind;
use kayak_ui::core::{
    render, rsx,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
    widget, Bound, Event, EventType, KayakContextRef, KeyCode, MutableBound, OnEvent,
};
use kayak_ui::widgets::{App as KApp, Text, Window};

use crate::state::GameState;
mod menu_ui;
use menu_ui::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum AnchorPoint {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

fn anchor(size: (f32, f32), parent: (f32, f32), anchor_point: AnchorPoint) -> (f32, f32) {
    match anchor_point {
        AnchorPoint::TopLeft => (0., 0.),
        AnchorPoint::TopCenter => (parent.0 / 2. - size.0 / 2., 0.),
        AnchorPoint::TopRight => (parent.0 - size.0, 0.),
        AnchorPoint::CenterLeft => (0., parent.1 / 2. - size.1 / 2.),
        AnchorPoint::Center => (parent.0 / 2. - size.0 / 2., parent.1 / 2. - size.1 / 2.),
        AnchorPoint::CenterRight => (parent.0 - size.0, parent.1 / 2. - size.1 / 2.),
        AnchorPoint::BottomLeft => (0., parent.1 - size.1),
        AnchorPoint::BottomCenter => (parent.1 / 2. - size.0 / 2., parent.1 - size.1),
        AnchorPoint::BottomRight => (parent.0 - size.0, parent.1 - size.1),
    }
}

fn handle_input(context: &mut KayakContextRef, event: &mut Event) {
    match event.event_type {
        EventType::KeyDown(event) => {
            if event.key() == KeyCode::Space {
                context.query_world::<ResMut<State<GameState>>, _, _>(swap);
            }
        }
        _ => {}
    };
}

fn create_main_menu(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.primary();
    let parent = (window.width(), window.height());
    let size = (300.0, 300.0);
    let position1 = anchor(size, parent, AnchorPoint::Center);
    let context = BevyContext::new(|context| {
        render! {
            <KApp on_event={Some(OnEvent::new(handle_input))}>
                <Window position={position1} size={size}>
                    <Text content={"GALACTIC WARS".to_string()} size={32.0} />
                    <MenuSelector />
                </Window>
            </KApp>
        }
    });

    commands.insert_resource(context);
}

fn create_options_menu(mut commands: Commands, windows: Res<Windows>) {
    let context = BevyContext::new(|context| {
        render! {
            <KApp on_event={Some(OnEvent::new(handle_input))}>
                <Text content={"Options".to_string()} size={32.0} />
            </KApp>
        }
    });

    commands.insert_resource(context);
}

fn create_play_menu(
    mut commands: Commands,
    mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("kenny/panel_brown.png");
    let panel_brown_handle = image_manager.get(&handle);
    let context = BevyContext::new(|context| {
        render! {
            <KApp on_event={Some(OnEvent::new(handle_input))}>
                <Text content={"Play".to_string()} size={32.0} />
            </KApp>
        }
    });

    commands.insert_resource(context);
}

fn ui_startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    // mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));
    // add font
    let main_font = asset_server.load("antiquity.kayak_font");
    font_mapping.add("Antiquity", main_font.clone());
    // add image
    //

    let context = BevyContext::new(|context| {
        render! {
            <KApp>
                <GameMenu/>
            </KApp>
        }
    });

    commands.insert_resource(context);
}

pub fn bind_gamestate(state: Res<State<GameState>>, binding: Res<Binding<GameState>>) {
    if state.is_changed() {
        binding.set(state.as_ref().clone());
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .insert_resource(bind(GameState::MainMenu))
            .add_startup_system(ui_startup)
            .add_system(bind_gamestate);
    }
}
