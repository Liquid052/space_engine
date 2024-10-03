use bevy::app::App;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};
use crate::prelude::LoadingStates;

// For loading asset collections before loading screen
pub trait BevyLoadCoreCollection {
    fn load_core_collection<A: AssetCollection>(&mut self) -> &mut Self;
}

impl BevyLoadCoreCollection for App {
    fn load_core_collection<A: AssetCollection>(&mut self) -> &mut Self {
        self.configure_loading_state(LoadingStateConfig::new(LoadingStates::CoreAssets)
            .load_collection::<A>());

        self
    }
}

//
pub trait BevyLoadCollection {
    fn load_collection<A: AssetCollection>(&mut self) -> &mut Self;
}

impl BevyLoadCollection for App {
    fn load_collection<A: AssetCollection>(&mut self) -> &mut Self {
        self.configure_loading_state(LoadingStateConfig::new(LoadingStates::ContentLoading)
            .load_collection::<A>());

        self
    }
}

