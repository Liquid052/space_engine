#![allow(dead_code)]
use crate::prelude::SceneNode;
use crate::resources::SceneLayerReg;
use crate::traits::{SceneConfig, SceneLayerHandler};
use bevy::prelude::*;

pub trait SceneTreeAppExt {
    fn scene_layer<T: Default + Component + SceneLayerHandler>(&mut self, name: impl Into<String>, config: SceneLayerConfig<T>) -> &mut App;
}
impl SceneTreeAppExt for App {
    fn scene_layer<T: Default + Component + SceneLayerHandler>(&mut self, name: impl Into<String>, config: SceneLayerConfig<T>) -> &mut App {
        config.apply(name.into(), self);

        self
    }
}


#[derive(Debug, Default, Clone)]
pub struct SceneLayerConfig<T: SceneLayerHandler> {
    marker: T,
    config: Config,
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    delete_on_save: bool,
}

impl<T: SceneLayerHandler> SceneConfig for SceneLayerConfig<T> {
    fn save(&self, node: &SceneNode, world: &mut World) {
        self.marker.save(node, &self.config, world);
    }
    fn load(&self, node: &SceneNode, world: &mut World) {
        self.marker.load(node, &self.config, world);
    }
}

impl<T: Default + Component + SceneLayerHandler> SceneLayerConfig<T> {
    pub fn new(marker: T) -> Self {
        Self {
            marker,
            config: default(),
        }
    }

    pub fn apply(self, layer_name: String, app: &mut App) {
        let world = app.world_mut();

        let layer_reg = world.resource_mut::<SceneLayerReg>();

        layer_reg.insert(layer_name, Box::new(self));
    }
}