use bevy::prelude::Handle;
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::styles::LayoutType;
use kayak_ui::core::Color;
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    use_state, widget, Bound, Children, EventType, MutableBound, OnEvent, WidgetProps,
};
use kayak_ui::widgets::{App as KApp, Button, Element, Text, Window};

#[widget]
pub fn Counter() {
    // Styles
    let text_styles = Style {
        ..Default::default()
    };

    let button_text_styles = Style {
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    // State implementation
    let (count, set_count, ..) = use_state!(0i32);

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::Click(..) => set_count(count + 1),
        _ => {}
    });

    let count_text = format!("Current Count: {}", count);
    rsx! {
        <>
            <Window position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Counter Example".to_string() }>
                <Text styles={Some(text_styles)} size={32.0} content={count_text} />
                <Button on_event={Some(on_event)}>
                    <Text size={24.0} content={"Count!".to_string()} />
                </Button>
            </Window>
        </>
    }
}

#[widget]
pub fn Menu() {
    rsx! {
        <>
            <Window>
            </Window>
        </>
    }
}

#[widget]
pub fn StateSwitcher() {
    rsx! {
        <Text content={"Press space to switch states!".to_string()} size={32.0} />
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
