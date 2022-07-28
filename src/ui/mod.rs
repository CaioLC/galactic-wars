use bevy::prelude::{App, AssetServer, Commands, Handle, Plugin, Res, ResMut, State, SystemSet};
use bevy::window::Windows;
use iyes_loopless::state::CurrentState;
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::{bind, Binding};
use kayak_ui::core::{
    render, rsx,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
    widget, Bound, Event, EventType, KayakContextRef, KeyCode, MutableBound, OnEvent,
};
use kayak_ui::widgets::{App as KApp, Text, Window};

use crate::state::{self, GameState};

mod menu_ui;
use menu_ui::*;

mod styles;

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
                <PauseMenu/>
            </KApp>
        }
    });

    commands.insert_resource(context);
}

pub fn bind_gamestate(state: Res<CurrentState<GameState>>, binding: Res<Binding<GameState>>) {
    if state.is_changed() {
        binding.set(state.0.clone());
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .insert_resource(bind(state::STARTING_GAME_STATE))
            .add_startup_system(ui_startup)
            .add_system(bind_gamestate);
    }
}
