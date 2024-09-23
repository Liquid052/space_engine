use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::helpers::generate_mod_index;
use crate::prelude::*;
use crate::systems::{cleanup, move_state, refresh};

pub struct CoreAssetsPlugin;

impl Plugin for CoreAssetsPlugin {
    fn build(&self, app: &mut App) {
        generate_mod_index();

        // plugins
        app.add_plugins(RonAssetPlugin::<ModIndex>::new(&["mod_index.ron"]))
            .add_plugins(RonAssetPlugin::<ModInfo>::new(&["mod_info.ron"]))
            .init_resource::<ProgressCounter>()
            .add_plugins(ProgressPlugin::new(LoadingStates::ModIndex).continue_to(LoadingStates::ModsMeta))
            .add_plugins(ProgressPlugin::new(LoadingStates::ModsMeta).continue_to(LoadingStates::ContentLoading))
            .add_plugins(ProgressPlugin::new(LoadingStates::ContentLoading).continue_to(LoadingStates::ContentProcessing))
            .add_plugins(ProgressPlugin::new(LoadingStates::ContentProcessing).continue_to(LoadingStates::Finished))
            // resources
            .init_resource::<EngineBuilding>()
            // assets
            .init_asset::<ModIndex>()
            .init_asset::<ModInfo>()
            // states
            .init_state::<LoadingStates>()
            .add_sub_state::<BuildingStates>()
            .add_computed_state::<BuildFinished>()
            .add_computed_state::<ContentLoaded>()
            .add_computed_state::<Loading>()
            // loading
            .add_loading_state(
                LoadingState::new(LoadingStates::CoreAssets)
                    .continue_to_state(LoadingStates::ModIndex)
            )
            .add_loading_state(
                LoadingState::new(LoadingStates::ModIndex)
                    .continue_to_state(LoadingStates::ModsMeta)
                    .load_collection::<ModPaths>()
            )
            .add_loading_state(
                LoadingState::new(LoadingStates::ModsMeta)
                    .continue_to_state(LoadingStates::ContentLoading)
                    .load_collection::<ModsCache>()
                    .init_resource::<Mods>()
            )
            .add_loading_state(
                LoadingState::new(LoadingStates::ContentLoading)
                    .continue_to_state(LoadingStates::ContentProcessing)
                    .load_collection::<ContentCache>()
                    .init_resource::<AssetDatabase>()
            )
            .add_loading_state(
                LoadingState::new(LoadingStates::ContentProcessing)
                    .continue_to_state(LoadingStates::Finished)
            )
            // systems
            // for managing state of BuildingState
            .add_systems(First, (move_state, refresh)
                .chain()
                .run_if(in_state(BuildingStates::Building))
            )
            .add_systems(OnEnter(BuildingStates::Finished), cleanup);
    }
}