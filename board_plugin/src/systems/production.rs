use crate::{FighterTimer, Planet};
use bevy::prelude::*;

pub fn produce_fighters(
    time: Res<Time>,
    mut timer: ResMut<FighterTimer>,
    mut query: Query<&mut Planet>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut planet in query.iter_mut() {
            planet.fighters += 1.;
            println!("fighter produced. Total: {:?}", planet.fighters)
        }
    };
}
