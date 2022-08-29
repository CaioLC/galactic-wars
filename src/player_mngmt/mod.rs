pub mod components;
pub mod resources;

use crate::game::components::config::InitGameSetup;
use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};
use components::*;
use resources::*;

pub struct PlayerManagementPlugin;

impl Plugin for PlayerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::RegisteredPlayers(HashMap::new()))
            .insert_resource(resources::AllegiancesToOthers(HashMap::new()))
            .insert_resource(resources::PlayerMoney(HashMap::new()))
            .add_startup_system(setup_players);
    }
}

fn setup_players(
    board_params: Res<InitGameSetup>,
    mut players: ResMut<RegisteredPlayers>,
    mut money: ResMut<PlayerMoney>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut allegiances_to_others: ResMut<AllegiancesToOthers>,
) {
    register_players(&mut players, &mut materials);
    let me = who_am_i("Caio", &players.0).expect("Could not find player"); // TODO: this won't work in real life.
    setup_initial_resources(&mut money, board_params.starting_resources, &players.0);
    setup_allegiances(me, &players.0, &mut allegiances_to_others);
}

fn who_am_i(name: &str, players_map: &HashMap<Uuid, PlayerDetails>) -> Option<Uuid> {
    for (k, v) in players_map.into_iter() {
        if name.to_string() == v.name {
            return Some(*k);
        }
    }
    None
}

fn setup_allegiances(
    me: Uuid,
    players_map: &HashMap<Uuid, PlayerDetails>,
    allegiances_to_others: &mut ResMut<AllegiancesToOthers>,
) {
    for (k, _) in players_map.into_iter() {
        if *k != me {
            allegiances_to_others
                .0
                .insert(*k, AllegianceStatus::Neutral);
        }
    }
}

fn setup_initial_resources(
    money_res: &mut ResMut<PlayerMoney>,
    start_amount: u32,
    players_map: &HashMap<Uuid, PlayerDetails>,
) {
    for (k, _) in players_map.into_iter() {
        money_res.0.insert(*k, start_amount);
    }
}

fn register_players(
    players: &mut ResMut<RegisteredPlayers>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    players.0.insert(
        Uuid::new_v4(),
        PlayerDetails {
            name: "Caio".to_string(),
            color: materials
                .add(StandardMaterial {
                    base_color: Color::BLUE,
                    unlit: true,
                    ..Default::default()
                })
                .into(),
        },
    );
    players.0.insert(
        Uuid::new_v4(),
        PlayerDetails {
            name: "Bob".to_string(),
            color: materials
                .add(StandardMaterial {
                    base_color: Color::RED,
                    unlit: true,
                    ..Default::default()
                })
                .into(),
        },
    );
}
