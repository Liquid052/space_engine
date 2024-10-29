use bevy::prelude::*;
use core_assets::prelude::*;
use engine_core::prelude::*;

fn main() {
    App::new()
        .add_plugins(CoreEnginePlugin::new())
        .add_systems(OnEnter(ContentLoaded), print_content)
        .run();
}

fn print_content(db: Res<AssetDatabase>) {
    info!("{:#?}", db);
}