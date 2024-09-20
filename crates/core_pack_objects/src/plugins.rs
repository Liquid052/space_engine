use bevy::app::{App, Plugin};
use crate::compos::{Pack, Unpacked, UnpackOnLoad};
use crate::prelude::TaggedObjDB;

pub struct PackObjectsPlugin;

impl Plugin for PackObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TaggedObjDB>()
            .register_type::<Pack>()
            .register_type::<UnpackOnLoad>()
            .register_type::<Unpacked>();
    }
}
