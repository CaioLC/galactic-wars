use bevy::prelude::*; 

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Selected;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Selectable;

pub struct IsSelecting {
    pub is_selecting: bool,
    pub mouse_enter: Option<Vec2>
}

#[derive(Component)]
pub struct SelectionBox;

pub struct SelectNearest(pub Entity);

pub struct SelectMany(pub Vec<Entity>);