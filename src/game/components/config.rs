use bevy::prelude::*;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardAssets::default())
            .insert_resource(BoardParams { no_of_planets: 1 });
    }
}

// #[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Default)]
pub struct BoardAssets {
    pub font: Handle<Font>,
    pub score_font: Handle<Font>,
    pub ship: GameMesh,
    pub laser: GameMesh,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct BoardParams {
    pub no_of_planets: u32,
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: Handle<StandardMaterial>,
}
