use bevy::prelude::*;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

#[derive(Default)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub score_font: Handle<Font>,
    pub ship: GameMesh,
    pub laser: GameMesh,
}

#[derive(Default)]
pub struct GameMesh {
    pub mesh: Handle<Mesh>,
    pub texture: Handle<StandardMaterial>,
}