use bevy::app::AppExit;
use bevy::prelude::{Res, World, EventWriter};
use iyes_loopless::state::NextState;

use kayak_ui::bevy::BevyContext;
use kayak_ui::core::styles::{Corner, Edge, LayoutType};
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    use_state, widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
};
use kayak_ui::core::{Binding, Color};
use kayak_ui::widgets::{Background, Button, Element, If, Text, Window, Image};

use super::styles as css;
use crate::state::{self, GameState};

#[widget]
pub fn GameMenu() {
    let show_menus = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::MainMenu
    };
    // Events
    let on_click_new_game = OnEvent::new(|ctx, event| match event.event_type {
        EventType::Click(..) => {
            dbg!("new game!");
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
            <Background styles={Some(css::container_style().with_style(css::center_align()))}>
                <Button
                    on_event={Some(on_click_new_game)}
                    styles={Some(css::button_style())}
                >
                    <Text size={20.0} content={"New Game".to_string()} />
                </Button>

                <Button
                    on_event={Some(on_click_settings)}
                    styles={Some(css::button_style())}
                >
                    <Text size={20.0} content={"Settings".to_string()} />
                </Button>

                <Button
                    on_event={Some(on_click_exit)}
                    styles={Some(css::button_style())}
                >
                    <Text size={20.0} content={"Exit".to_string()} />
                </Button>
            </Background>
        </If>
    }
}

#[widget]
pub fn PauseMenu() {
    let show_pause = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::Pause
    };
    let pause_ribbon = Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),

        height: StyleProp::Value(Units::Percentage(40.0)),
        width: StyleProp::Value(Units::Percentage(100.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(0.6, 0.6, 0.6, 0.4)),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    };

    // RSX
    rsx! {
        <If condition={show_pause}>
            <Background styles={Some(pause_ribbon)}>
                <Text size={20.0} content={"GAME PAUSED".to_string()} />
            </Background>
        </If>
    }
}

#[widget]
pub fn InGameUI() {
    let in_game = {
        let gamestate = context.query_world::<Res<Binding<GameState>>, _, _>(|state| state.clone());
        context.bind(&gamestate);
        gamestate.get() == GameState::InGame
    };
    rsx! {
        <If condition={in_game}>
            <TopNavBar/>
            // <MultiplayerAndLog/>
            // <GroupsBar/>
            // <MiniMap/>
            // <ChatBar/>
        </If>
    }
}

#[widget]
pub fn TopNavBar() {
    let nav_bar = Style {
        width: StyleProp::Value(Units::Percentage(100.)),
        height: StyleProp::Value(Units::Pixels(40.)), 
        ..Default::default()
    };

    rsx! {
        <Background styles={Some(nav_bar.with_style(css::center_align()))}>
            <Resources/>
            <ShipsAndPlanetsDetail/>
            <QuickMenu/>
        </Background>
    }
}

#[widget]
pub fn Resources() {
    rsx! {
        <Text size={20.0} content={"5000".to_string()} />
    }
}

#[widget]
pub fn ShipsAndPlanetsDetail() {
    rsx! {
        <Element>
            <Text size={20.0} content={"figthers".to_string()} />
            <Text size={20.0} content={"traders".to_string()} />
            <Text size={20.0} content={"dreadnoughts".to_string()} />
            <Text size={20.0} content={"planets".to_string()} />
        </Element>
    }
}

#[widget]
pub fn QuickMenu (){
    rsx! {
        <Element>
            <Text size={20.0} content={"A".to_string()} />
            <Text size={20.0} content={"N".to_string()} />
            <Text size={20.0} content={"S".to_string()} />
            <Text size={20.0} content={"M".to_string()} />
        </Element>
    }
}

#[widget]
pub fn IconAndText() {
    rsx! {
        <Element>
            // <Image styles={Some()} handle={}/>
            <Text size={20.0} content={"5000".to_string()} />
        </Element>
    }
}
// #[widget]
// pub fn MultiplayerAndLog() {}

// #[widget]
// pub fn GroupsBar() {}

// #[widget]
// pub fn MiniMap() {}

// #[widget]
// pub fn ChatBar() {}
