use bevy::{prelude::*, utils::HashMap};

#[derive(Clone, PartialEq)]
pub struct FightersDeployed(pub u32);

#[derive(Clone, PartialEq)]
pub struct FightersStored(pub u32);

#[derive(Clone, PartialEq)]
pub struct TotalTraders(pub u32);

#[derive(Clone, PartialEq)]
pub struct TotalDreadnoughts(pub u32);

#[derive(Clone, PartialEq)]
pub struct TotalPlanets(pub u32);

pub struct MovingFleets(pub HashMap<String, Vec<Entity>>);
