use bevy::prelude::*;
use bevy_asset_loader::asset_collection::{AssetCollection, AssetCollectionApp};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "img/Background.png")]
    pub bg_main: Handle<Image>,
    #[asset(path = "img/bg_light.png")]
    pub bg_light: Handle<Image>,
    #[asset(path = "img/bg_dark.png")]
    pub bg_dark: Handle<Image>,
    #[asset(path = "img/Planet_31.png")]
    pub figther_handle: Handle<Image>,
    #[asset(path = "img/Planet_43.png")]
    pub trader_handle: Handle<Image>,
    #[asset(path = "img/Planet_59.png")]
    pub dreadn_handle: Handle<Image>,
    #[asset(path = "img/Planet2_7.png")]
    pub planet_handle: Handle<Image>,
    #[asset(path = "img/generic-rpg-vendor.png")]
    pub rpg_vendor: Handle<Image>,
}
