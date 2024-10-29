use bevy::prelude::*;

use core_assets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_plugins(CoreAssetsPlugin)
        .add_systems(OnEnter(ContentLoaded), print_content)
        .run();
}

fn print_content(db: Res<AssetDatabase>) {
    info!("{:#?}", db);
}
