use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

pub const STARTING_GAME_STATE: GameState = GameState::MainMenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Options,
    InGame,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // TODO: this 'stage_before' should be moved to GamePlugin
        app
            // .add_stage_before(
            //     CoreStage::Update,
            //     "fighter_producer_tick",
            //     FixedTimestepStage::new(Duration::from_secs_f32(2.0))
            //         .with_stage(SystemStage::parallel().with_system(producer_tick)),
            // )
            .add_loopless_state(STARTING_GAME_STATE)
            .add_system(stage_key_bindings);
        // .add_enter_system(&GameState::InGame, setup_game);
    }
}

// TODO: WIRE THIS FIXED STAGE TO ACTUAL FIGHTER PRODUCTION
// fn producer_tick() {
//     dbg!("tick!");
// }

fn stage_key_bindings(mut commands: Commands, kb_input: Res<Input<KeyCode>>) {
    if kb_input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

// fn setup_game(mut commands: Commands) {
//     dbg!("reset_game!");
//     commands.insert_resource(NextState(GameState::InGame));
// }
