use super::components::{
    characteristics::DestinationEnum,
    players::{AllegianceStatus, PlayerDetails},
};
use bevy::{
    prelude::{Entity, Handle, StandardMaterial},
    utils::{HashMap, Uuid},
};

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

pub struct PlayersRes(pub HashMap<Uuid, PlayerDetails>);
pub struct PlayersColor(pub HashMap<Uuid, Handle<StandardMaterial>>);
pub struct MovingFleets(pub HashMap<String, Vec<Entity>>);

// These need to be local
pub struct AllegiancesToOthers(pub HashMap<String, AllegianceStatus>);
pub struct AllegiancesToMe(pub HashMap<String, AllegianceStatus>);
