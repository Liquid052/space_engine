use bevy::prelude::*;
use crate::prelude::*;

pub struct NamingPlugin;

impl Plugin for NamingPlugin {
    fn build(&self, app: &mut App) {
        // resources
        app.init_resource::<NameReg>()
            //
            .register_type::<NameReg>()
            .register_type::<UniquelyNamed>()
            .register_type::<Name>();
    }
}