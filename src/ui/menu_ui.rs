use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Handle, Res, ResMut, World};
use iyes_loopless::state::NextState;

use kayak_ui::bevy::{BevyContext, ImageManager};
use kayak_ui::core::styles::{Corner, Edge, LayoutType};
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    use_state, widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
};
use kayak_ui::core::{Binding, Color};
use kayak_ui::widgets::{Background, Button, Element, If, Image, Text, Window};

use super::styles::*;
use crate::state::{self, GameState};

#[widget]
pub fn GameMenu() {
    let container_style = container_style()
        .with_style(bg_primary())
        .with_style(center());
    let show_menus = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::MainMenu
    };
    // Events
    let on_click_new_game = OnEvent::new(|ctx, event| match event.event_type {
        EventType::Click(..) => {
            // dbg!("new game!");
            let mut world = ctx.get_global_mut::<World>().unwrap();
            world.insert_resource(NextState(GameState::InGame));
        }
        _ => {}
    });

    let on_click_settings = OnEvent::new(|_, event| match event.event_type {
        EventType::Click(..) => {
            dbg!("Settings");
        }
        _ => {}
    });

    let on_click_exit = OnEvent::new(|ctx, event| match event.event_type {
        EventType::Click(..) => ctx.query_world::<EventWriter<AppExit>, _, _>(|mut exit| {
            exit.send(AppExit);
        }),
        _ => {}
    });

    // RSX
    rsx! {
        <If condition={show_menus}>
            <Background styles={Some(container_style)}
            >
                <Button
                    on_event={Some(on_click_new_game)}
                    styles={Some(button_style())}
                >
                    <Text size={20.0} content={"New Game".to_string()} />
                </Button>

                <Button
                    on_event={Some(on_click_settings)}
                    styles={Some(button_style())}
                >
                    <Text size={20.0} content={"Settings".to_string()} />
                </Button>

                <Button
                    on_event={Some(on_click_exit)}
                    styles={Some(button_style())}
                >
                    <Text size={20.0} content={"Exit".to_string()} />
                </Button>
            </Background>
        </If>
    }
}
