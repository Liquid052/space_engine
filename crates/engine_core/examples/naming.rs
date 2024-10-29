use bevy::log::LogPlugin;
use bevy::prelude::*;
use engine_core::bundles::NameBundle;
use engine_core::commands::RenameEntity;
use engine_core::prelude::*;

fn main() {
    App::new()
        .add_plugins(CoreEnginePlugin::new())
        // systems
        .add_systems(Startup, (setup, print).chain())
        .add_systems(Update, update_name.run_if(nth_frame_once::<10>))
        .add_systems(Update, (print, remove).chain().run_if(nth_frame_once::<20>))
        .add_systems(Update, print.run_if(nth_frame_once::<30>))
        .run();
}


fn setup(mut com: Commands) {
    com.spawn(NameBundle::new("Adam"));
}

fn update_name(mut names: Query<Entity, (With<Name>, With<UniquelyNamed>)>, mut commands: Commands) {
    names.iter_mut()
        .for_each(|ent| {
            commands.rename(ent, "Eve");
        });
}

fn remove(mut com: Commands, query: Query<Entity, (With<Name>, With<UniquelyNamed>)>) {
    for ent in query.iter() {
        com.entity(ent).remove::<UniquelyNamed>();
    }
}
fn print(names: Res<NameReg>) {
    info!("{:?}", names);
}