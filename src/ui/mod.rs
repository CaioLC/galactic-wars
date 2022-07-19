use bevy::prelude::{App, AssetServer, Commands, Handle, Plugin, Res, ResMut, State, SystemSet};
use bevy::window::{WindowDescriptor, Windows};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, ImageManager, UICameraBundle};
use kayak_ui::core::{
    render, rsx,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
    widget, Bound, Event, EventType, KayakContextRef, KeyCode, MutableBound, OnEvent,
};
use kayak_ui::widgets::{App as KApp, NinePatch, Text, Window};

mod menu_ui;
use menu_ui::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    MainMenu,
    Options,
    Play,
}

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

fn swap(mut state: ResMut<State<GameState>>) {
    if *state.current() == GameState::MainMenu {
        let _ = state.set(GameState::Options);
    } else if *state.current() == GameState::Options {
        let _ = state.set(GameState::Play);
    } else {
        let _ = state.set(GameState::MainMenu);
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
                <StateSwitcher />
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
                <StateSwitcher />
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
                <StateSwitcher />
            </KApp>
        }
    });

    commands.insert_resource(context);
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    mut image_manager: ResMut<ImageManager>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));
    // add font
    let main_font = asset_server.load("antiquity.kayak_font");
    font_mapping.add("Antiquity", main_font.clone());
    // add image
}

fn destroy(mut commands: Commands) {
    commands.remove_resource::<BevyContext>();
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainMenu)
            .add_plugin(BevyKayakUIPlugin)
            .add_startup_system(startup)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy))
            .add_system_set(
                SystemSet::on_enter(GameState::Options).with_system(create_options_menu),
            )
            .add_system_set(SystemSet::on_exit(GameState::Options).with_system(destroy))
            .add_system_set(SystemSet::on_enter(GameState::Play).with_system(create_play_menu))
            .add_system_set(SystemSet::on_exit(GameState::Play).with_system(destroy));
    }
}
