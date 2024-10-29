mod naming;

use crate::prelude::*;
use bevy::prelude::*;
use core_assets::prelude::*;
use core_pack_objects::prelude::*;
use core_world_streaming::plugins::WorldStreamingPlugin;

use crate::attributes::SpritePathAttribute;
// export
pub use naming::*;

/// Main plugin
pub struct CoreEnginePlugin {
    minimal: bool,
    direct_load: Option<String>,
}

impl CoreEnginePlugin {
    pub fn new() -> Self {
        Self {
            minimal:     false,
            direct_load: None
        }
    }

    pub fn minimal(mut self) -> Self {
        self.minimal = true;

        self
    }

    /// When you want to load a save immediately after app's assets (BuildingState) are loaded and built
    pub fn direct_load(mut self, path: impl Into<String>) -> Self {
        self.direct_load = Some(path.into());

        self
    }
}


impl Plugin for CoreEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreAssetsPlugin)
            .add_plugins(PackObjectsPlugin)
            .add_plugins(NamingPlugin)
            .add_plugins(WorldStreamingPlugin)
            .add_plugins(CameraManagerPlugin)
            .add_sub_state::<AppState>()
            .add_computed_state::<InGame>()
            .add_computed_state::<Running>()
            .add_computed_state::<Paused>()
            // attributes
            .add_pack_attribute(SpritePathAttribute);
    }
}

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetPlugin::default())
            .add_plugins(MinimalPlugins);
    }
}