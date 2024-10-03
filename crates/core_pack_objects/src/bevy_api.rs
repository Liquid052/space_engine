use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;


/// for adding api to Bevy
pub trait BevyTagExt {
    fn add_pack<T: PackTransformer>(&mut self, builder: T) -> &mut Self;

    fn add_pack_attribute<T: GeneralTransformer>(&mut self, builder: T) -> &mut Self;
}

impl BevyTagExt for App {
    fn add_pack<T: PackTransformer>(&mut self, builder: T) -> &mut Self {
        if !self.is_plugin_added::<PackObjectsPlugin>() {
           self.add_plugins(PackObjectsPlugin);
        }

        let res = self.world().resource::<TaggedObjDB>();

        res.register(builder);

        self
    }

    fn add_pack_attribute<T: GeneralTransformer>(&mut self, builder: T) -> &mut Self {
        if !self.is_plugin_added::<PackObjectsPlugin>() {
            self.add_plugins(PackObjectsPlugin);
        }

        let res = self.world().resource::<TaggedObjDB>();

        res.register_general(builder);

        self
    }
}
