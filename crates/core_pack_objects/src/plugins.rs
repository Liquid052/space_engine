use crate::components::{Pack, UnpackOnLoad, Unpacked};
use crate::prelude::TaggedObjDB;
use bevy::app::{App, Plugin};

pub struct PackObjectsPlugin;

impl Plugin for PackObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TaggedObjDB>()
            .register_type::<Pack>()
            .register_type::<UnpackOnLoad>()
            .register_type::<Unpacked>();
    }
}
