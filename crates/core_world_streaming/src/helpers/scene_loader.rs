use crate::components::SceneNode;
use crate::prelude::{ProcessedScene, SceneProcessor};
use bevy::prelude::*;

pub struct SceneLoader<'w> {
    world: &'w mut World,
    node: &'w SceneNode,
    hooks: Vec<Box<dyn Fn(&mut World, Scene) -> Result<Scene, String> + Send + Sync + 'static>>,
}

impl<'w> SceneLoader<'w> {
    pub fn new(world: &'w mut World, node: &'w SceneNode) -> Self {
        Self {
            world,
            node,
            hooks: default(),
        }
    }
    pub fn hook<F: Fn(&mut World, Scene) -> Result<Scene, String> + Send + Sync + 'static>(mut self, hook: F) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    pub fn load(self) {
        let mut scene_processor = self.world.resource::<SceneProcessor>().clone();
        let asset_server = self.world.resource::<AssetServer>().clone();

        let path = self.node.load_path_unchecked();
        let dyn_scene = asset_server.load(path.to_string());

        let mut processed_scene = ProcessedScene::new(dyn_scene);

        self.hooks.into_iter().for_each(|boxed_fn| {
            processed_scene.hook_dyn(boxed_fn);
        });

        scene_processor.add_subscene(processed_scene);
    }
}


