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

mod ingame_ui;
use ingame_ui::*;

mod styles;

fn ui_startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));
    // add font
    // let main_font = asset_server.load("fonts/antiquity.kayak_font");
    // font_mapping.add("Antiquity", main_font.clone());
    // add image

    let context = BevyContext::new(|context| {
        render! {
            <KApp>
                <GameMenu/>
                <PauseMenu/>
                <InGameUI/>
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
