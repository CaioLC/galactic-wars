pub mod components;
pub mod obj;
pub mod resources;
mod systems;
pub mod utils;

use std::{f32::consts::PI, time::Duration};

use bevy::{
    prelude::*,
    utils::{HashMap, Uuid},
};
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::GameState;

use components::{
    characteristics::*,
    config::*,
    players::{AllegianceStatus, PlayerDetails},
};
use obj::{spawn_planet, spawn_ship};
use resources::{game_obj_res::*, game_status_res::*, player_res::*};
use systems::*;

use self::{
    components::config,
    utils::layers_util::{get_z, Layers},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(config::ConfigPlugin)
            .insert_resource(FightersDeployed(0))
            .insert_resource(FightersStored(0))
            .insert_resource(TotalTraders(0))
            .insert_resource(TotalDreadnoughts(0))
            .insert_resource(TotalPlanets(0))
            .insert_resource(MovingFleets(HashMap::new()))
            .insert_resource(GameStatus(GameStatusEnum::Uninitialized))
            .insert_resource(RegisteredPlayers(HashMap::new()))
            .insert_resource(AllegiancesToOthers(HashMap::new()))
            .insert_resource(PlayerMoney(HashMap::new()))
            .add_event::<TakeOwnership>()
            .add_event::<ArrivedAtDestination>()
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(TextMeshPlugin)
            .add_startup_system(setup)
            .add_stage_before(
                CoreStage::Update,
                "fighter_producer_tick",
                FixedTimestepStage::new(Duration::from_secs_f32(2.0)) //TODO: this should be configurable
                    .with_stage(SystemStage::parallel().with_system(production::production_tick)),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(production::fighter_enters_planet)
                    .with_system(production::deploy_fighters)
                    .with_system(production::update_count_mesh)
                    .with_system(production::take_planet_ownership)
                    .with_system(movement::turn_to_destination)
                    .with_system(movement::move_to_destination)
                    .with_system(movement::set_destination)
                    .with_system(movement::remove_destination)
                    .with_system(movement::damping_shift)
                    .with_system(movement::collision_avoidance)
                    .with_system(combat::bullet_hit)
                    .with_system(combat::fire_bullet)
                    .with_system(combat::despawn_bullet)
                    // this should be moved to a system set that runs at the end of frame
                    .with_system(production::count_fighters_deployed)
                    .with_system(production::count_fighters_stored)
                    .with_system(production::count_traders)
                    .into(),
            );

        #[cfg(feature = "debug")]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    board_params: Res<InitGameSetup>,
    mut players: ResMut<RegisteredPlayers>,
    mut game_status: ResMut<GameStatus>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut money: ResMut<PlayerMoney>,
    mut allegiances_to_others: ResMut<AllegiancesToOthers>,
) {
    // setup players
    setup_players(
        &board_params,
        &mut players,
        &mut money,
        &mut materials,
        &mut allegiances_to_others,
    );

    // Set Player starting planets
    let mut placed_planets: Vec<Vec3> = Vec::new();
    for (pk, pd) in players.0.iter() {
        let mut finding_space = true;
        let mut transf = random_planet_pos(&board_params);
        while finding_space {
            let mut conflict_planet = None;
            for planet in placed_planets.iter() {
                if planet.distance(transf.translation)
                    < 5. * planet_type_to_radius(&PlanetType::Capital)
                {
                    conflict_planet = Some(planet);
                    break;
                }
            }
            match conflict_planet {
                // keep re-running random_planet_pos while there is conflict between planets
                Some(_) => transf = random_planet_pos(&board_params),
                None => finding_space = false,
            }
        }
        placed_planets.push(transf.translation);
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            asset_server.load("fonts/ShareTechMono.ttf"),
            // Planet config
            PlanetType::Capital,
            transf,
            Some(*pk),
            pd.color.clone(),
            0.,
        );
    }

    // Set other planets
    let non_player_color = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        unlit: true,
        ..Default::default()
    });

    for _ in 0..board_params.no_of_planets {
        let mut finding_space = true;
        let mut transf = random_planet_pos(&board_params);
        let planet_type = rand::random::<PlanetType>();
        while finding_space {
            let mut conflict_planet = None;
            for planet in placed_planets.iter() {
                if planet.distance(transf.translation) < 2. * planet_type_to_radius(&planet_type) {
                    conflict_planet = Some(planet);
                    break;
                }
            }
            match conflict_planet {
                // keep re-running random_planet_pos while there is conflict between planets
                Some(_) => transf = random_planet_pos(&board_params),
                None => finding_space = false,
            }
        }
        placed_planets.push(transf.translation);
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            asset_server.load("fonts/ShareTechMono.ttf"),
            // Planet config
            planet_type,
            transf,
            None,
            non_player_color.clone(),
            20.,
        );
    }

    let seed = 3552441;
    game_status.0 = GameStatusEnum::Started(seed);
}

fn setup_players(
    board_params: &Res<InitGameSetup>,
    players: &mut ResMut<RegisteredPlayers>,
    money: &mut ResMut<PlayerMoney>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    allegiances_to_others: &mut ResMut<AllegiancesToOthers>,
) {
    register_players(players, materials);
    let me = who_am_i("Caio", &players.0).expect("Could not find player"); // TODO: this won't work in real life.
    setup_initial_resources(money, board_params.starting_resources, &players.0);
    setup_allegiances(me, &players.0, allegiances_to_others);
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

fn random_planet_pos(game_config: &Res<InitGameSetup>) -> Transform {
    let z = get_z(Layers::Planets);
    let radius = components::config::galaxy_size_to_radius(&game_config.galaxy_size);
    let dist = rand::random::<f32>();
    let angle = rand::random::<f32>() * PI * 2.0;
    let x = angle.cos() * dist * radius;
    let y = angle.sin() * dist * radius;
    Transform::from_xyz(x, y, z)
}

// fn random_player_pos() -> Transform {}
