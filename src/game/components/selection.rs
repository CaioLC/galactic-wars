use bevy::prelude::*; 

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Selected;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Selectable;


pub struct SelectNearest(pub Entity);

pub struct SelectMany(pub Vec<Entity>);