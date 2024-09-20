extern crate engine;
extern crate bevy;

use bevy::prelude::*;
use engine::prelude::*;

fn main(){
    App::new()
        .add_plugins(EnginePlugin::new("Print content example"))
        .add_systems(OnEnter(ContentLoaded), print_content)
        .run();
}

fn print_content(db: Res<AssetDatabase>) {
    info!("{:#?}", db);
}
