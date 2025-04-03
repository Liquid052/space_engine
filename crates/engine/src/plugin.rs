extern crate bevy;

use crate::prelude::LoadingStates;
use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::prelude::Window;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy_asset_loader::prelude::{AssetCollection, ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};
use engine_core::prelude::CoreEnginePlugin;
use space::plugins::SpacePlugin;
use utils::plugin::UtilityPlugins;

// entry point
#[derive(Default)]
pub struct EnginePlugin {
    pub name:             String,
    pub enable_space:     bool,

    #[doc(hidden)]
    loader_injection: LoaderInjection,
}

// for adding external dependencies into bevy_asset_loader


impl EnginePlugin {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }
    pub fn enable_space(mut self) -> Self {
        self.enable_space = true;

        self
    }

    pub fn load<A: AssetCollection>(mut self) -> Self {
        self.loader_injection.vec.push(Box::new(|app| {
            app.configure_loading_state(
                LoadingStateConfig::new(LoadingStates::CoreAssets)
                    .load_collection::<A>()
            );
        }));

        self
    }
}

#[derive(Default)]
struct LoaderInjection {
    pub vec: Vec<Box<dyn Fn(&mut App) + Sync + Send>>
}

impl Plugin for LoaderInjection {
    fn build(&self, app: &mut App) {
        self.vec.iter().for_each(|boxed_fn| boxed_fn(app))
    }
}

impl PluginGroup for EnginePlugin {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins)
            .add(CoreEnginePlugin::default())
            .add(self.loader_injection);

        group = group.set(WindowPlugin {
            primary_window: Some(Window {
                title: self.name,
                ..Default::default()
            }),
            ..default()
        });

        group = group.add_group(UtilityPlugins);

        if self.enable_space {
            group = group.add(SpacePlugin {
                draw_enabled: true,
                camera_enabled: false,
                cam_background_enabled: false,
                auto_soi_update: false,
                cam_target: None,
            });
        }
        
        group
    }
}
