use bevy::utils::{Uuid, HashMap};
use super::components::players::AllegianceStatus;

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

pub struct PlayersRes(pub HashMap<String, Uuid>);
// These need to be local
pub struct AllegiancesToOthers(pub HashMap<String, AllegianceStatus>);
pub struct AllegiancesToMe(pub HashMap<String, AllegianceStatus>);