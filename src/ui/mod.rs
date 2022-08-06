mod menu_ui;
mod ingame_ui;
mod styles;
mod generics;

use bevy::prelude::{App, AssetServer, Commands, Plugin, Res, ResMut};
use iyes_loopless::state::CurrentState;

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{bind, Binding};
use kayak_ui::core::{
    render,
    MutableBound,
};
use kayak_ui::widgets::App as KApp;


use crate::state::{self, GameState};
use menu_ui::*;
use ingame_ui::*;

fn ui_startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    // mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

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
