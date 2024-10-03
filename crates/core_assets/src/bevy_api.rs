use bevy::app::App;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};
use crate::prelude::LoadingStates;


pub trait CollectionLoadingExt {
    // For loading asset collections before loading screen
    fn load_core_collection<A: AssetCollection>(&mut self) -> &mut Self;
    fn load_collection<A: AssetCollection>(&mut self) -> &mut Self;
}

impl CollectionLoadingExt for App {
    fn load_core_collection<A: AssetCollection>(&mut self) -> &mut Self {
        self.configure_loading_state(LoadingStateConfig::new(LoadingStates::CoreAssets)
            .load_collection::<A>());

        self
    }

    fn load_collection<A: AssetCollection>(&mut self) -> &mut Self {
        self.configure_loading_state(LoadingStateConfig::new(LoadingStates::ContentLoading)
            .load_collection::<A>());

        self
    }
}
