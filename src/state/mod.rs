use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Options,
    Play,
}

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            "fighter_producer_tick",
            FixedTimestepStage::new(Duration::from_secs_f32(2.0))
                .with_stage(SystemStage::parallel().with_system(producer_tick)),
        );
    }
}

// TODO: WIRE THIS FIXED STAGE TO ACTUAL FIGHTER PRODUCTION
fn producer_tick() {
    dbg!("tick!");
}
