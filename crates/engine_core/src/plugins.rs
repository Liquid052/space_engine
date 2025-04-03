mod naming;

use crate::prelude::*;
use bevy::prelude::*;
use core_assets::prelude::*;
use core_camera::prelude::CameraManagerPlugin;
// use core_pack_objects::prelude::*;
use core_utils::prelude::CoreUtilsPlugin;

// export
use naming::*;


#[derive(Default)]
pub struct CoreEnginePlugin;

impl CoreEnginePlugin {

}


impl Plugin for CoreEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreAssetsPlugin)
            // .add_plugins(PackObjectsPlugin)
            .add_plugins(NamingPlugin)
            // .add_plugins(WorldStreamingPlugin)
            // .add_plugins(CoreWorldGenerationPlugin)
            .add_plugins(CameraManagerPlugin)
            .add_plugins(CoreUtilsPlugin)
            .add_sub_state::<AppState>()
            .add_computed_state::<InGame>()
            .add_computed_state::<Running>()
            .add_computed_state::<Paused>();
            // attributes
            // .add_pack_attribute(SpritePathAttribute);
    }
}

#[doc(hidden)]
pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetPlugin::default())
            .add_plugins(MinimalPlugins);
    }
}