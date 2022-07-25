use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Handle};
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::styles::{Corner, Edge, LayoutType};
use kayak_ui::core::Color;
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    use_state, widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
};
use kayak_ui::widgets::{App as KApp, Background, Button, Element, Text, Window};

#[widget]
pub fn GameMenu() {
    let container_styles = Style {
        background_color: StyleProp::Value(Color::WHITE),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        right: StyleProp::Value(Units::Stretch(1.0)),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };

    let button_styles = Style {
        background_color: StyleProp::Value(Color::BLACK),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    let on_click_new_game = OnEvent::new(|_, event| match event.event_type {
        EventType::Click(..) => {
            dbg!("new game!");
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

    rsx! {
        <Background styles={Some(container_styles)}>
            <Button
                on_event={Some(on_click_new_game)}
                styles={Some(button_styles)}
            >
                <Text size={20.0} content={"New Game".to_string()} />
            </Button>

            <Button
                on_event={Some(on_click_settings)}
                styles={Some(button_styles)}
            >
                <Text size={20.0} content={"Settings".to_string()} />
            </Button>

            <Button
                on_event={Some(on_click_exit)}
                styles={Some(button_styles)}
            >
                <Text size={20.0} content={"Exit".to_string()} />
            </Button>
        </Background>
    }
}

#[widget]
pub fn MenuSelector() {
    let button_container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        width: StyleProp::Value(Units::Percentage(50.)),
        height: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Percentage(20.)),
        ..Default::default()
    };

    rsx! {
        <Element styles={Some(button_container_style)}>
            <Text content={"this is button1".to_string()} size={15.0} />
            <Text content={"this is button2".to_string()} size={20.0} />
            <Text content={"this is button3".to_string()} size={32.0} />
            <Text content={"this is button4".to_string()} size={45.0} />
        </Element>
    }
}
