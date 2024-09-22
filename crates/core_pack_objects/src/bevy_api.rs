use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;

pub trait PackTransformer: Send + Debug + Sync + 'static {
    fn type_name(&self) -> &str;

    // list of attributes/tags that are required by the builder. Also for type checks of Tag variants
    fn required(&self) -> &[(&str, TagType)] {
        &[]
    }
    fn optional(&self) -> &[(&str, TagType)] {
        &[]
    }

    fn unpack(&self, builder: &mut EntityTransformer);


    fn pack(&self, builder: &mut EntityTransformer);
}

pub trait GeneralTransformer: Send + Debug + Sync + 'static {
    fn attribute(&self) -> (&str, TagType);

    fn unpack(&self, builder: &mut EntityTransformer);


    fn pack(&self, builder: &mut EntityTransformer);
}


/// for adding api to Bevy
pub trait BevyTagApi {
    fn add_pack<T: PackTransformer>(&mut self, builder: T) -> &mut Self;

    fn add_pack_attribute<T: GeneralTransformer>(&mut self, builder: T) -> &mut Self;
}

impl BevyTagApi for App {
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
