use bevy::prelude::*;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardAssets::default())
            .insert_resource(InitGameSetup {
                no_of_planets: 3,
                starting_resources: 500,
                epoch_seconds: 30,
                galaxy_size: Galaxy::Small,
            });
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub enum Galaxy {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Ludicrous,
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
pub struct InitGameSetup {
    pub no_of_planets: u32,
    pub starting_resources: u32,
    pub epoch_seconds: u32,
    pub galaxy_size: Galaxy,
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: Handle<StandardMaterial>,
}
