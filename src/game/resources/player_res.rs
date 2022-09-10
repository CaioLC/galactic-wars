use super::super::components::players::*;
use bevy::{
    prelude::*,
    utils::{HashMap, Uuid},
};

pub struct RegisteredPlayers(pub HashMap<Uuid, PlayerDetails>);
pub struct PlayersColor(pub HashMap<Uuid, Handle<StandardMaterial>>);
pub struct PlayerMoney(pub HashMap<Uuid, u32>);

// These need to be local
pub struct AllegiancesToOthers(pub HashMap<Uuid, AllegianceStatus>);
