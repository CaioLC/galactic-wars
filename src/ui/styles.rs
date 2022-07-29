use std::{iter::Sum, ops::Add};

use kayak_ui::core::{
    styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
    Color,
};

pub fn container_style() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),

        height: StyleProp::Value(Units::Pixels(500.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::WHITE),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    }
}

pub fn button_style() -> Style {
    Style {
        background_color: StyleProp::Value(Color::BLACK),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}

impl Add for Style {
    type Output = Style;

    fn add(self, rhs: Style) -> Style {
        let mut resulting_style = Style::default();
        for i in Style::default().keys.iter() {
            if resulting_style.i == StyleProp::Default {
                resulting_style.i = rhs.i;
            }
        }
        resulting_style
    }
}

pub fn center_align() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}

pub fn main_background_color() -> Style {
    Style {
        background_color: StyleProp::Value(Color::BLACK),
        ..Default::default()
    }
}

pub fn complex_style() -> Style {
    center_align() + main_background_color()
}
