use crate::app_api::Config;
use crate::prelude::SceneNode;
use bevy::prelude::World;
use std::fmt::Debug;


pub trait SceneConfig: Send + Sync + Debug {
    fn save(&self, node: &SceneNode, world: &mut World);
    fn load(&self, path: &SceneNode, world: &mut World);
}

pub trait SceneLayerHandler: Send + Sync + Debug {
    fn save(&self, path: &SceneNode, cfg: &Config, world: &mut World);
    fn load(&self, path: &SceneNode, cfg: &Config, world: &mut World);
}