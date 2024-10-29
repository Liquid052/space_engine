use bevy::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

// aliases
type SceneHook = Box<dyn Fn(&mut World, Scene) -> Result<Scene, String> + Send + Sync>;

// Resource for processing scenes
#[derive(Resource, Default, Clone)]
pub struct SceneProcessor {
    pub vec: Arc<Mutex<Vec<ProcessedScene>>>,
    pub sub_scenes: Arc<Mutex<Vec<ProcessedScene>>>,
}

impl SceneProcessor {
    pub fn add(&mut self, scene: ProcessedScene) {
        self.vec.lock().unwrap().push(scene);
    }
    pub fn lock(&self) -> MutexGuard<Vec<ProcessedScene>> {
        self.vec.lock().unwrap()
    }

    pub fn add_subscene(&mut self, scene: ProcessedScene) {
        self.sub_scenes.lock().unwrap().push(scene);
    }
    // move subscenes to vec
    pub fn move_subscenes(&mut self) {
        let mut sub_scenes = self.sub_scenes.lock().unwrap();
        let mut scenes = self.vec.lock().unwrap();
        scenes.append(&mut sub_scenes);
    }
}

// Container for a scene to be processed
pub struct ProcessedScene {
    pub dyn_scene: Handle<DynamicScene>,
    pub hooks: Vec<SceneHook>,
}

impl ProcessedScene {
    pub fn new(dyn_scene: Handle<DynamicScene>) -> Self {
        let processed_scene = Self {
            dyn_scene,
            hooks: Vec::new(),
        };

        processed_scene
    }
    pub fn hook<F: Fn(&mut World, Scene) -> Result<Scene, String> + Send + Sync + 'static>(mut self, hook: F) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }
    pub fn hook_dyn(&mut self, hook: Box<dyn Fn(&mut World, Scene) -> Result<Scene, String> + Send + Sync + 'static>) {
        self.hooks.push(hook);
    }
}

