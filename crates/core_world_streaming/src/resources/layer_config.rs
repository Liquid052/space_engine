use crate::traits::SceneConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Default, Debug, Clone, Resource)]
pub struct SceneLayerReg {
    pub layers: Arc<RwLock<HashMap<String, Box<dyn SceneConfig>>>>,

    pub current_save: Option<String>,
    pub write_dir: Option<String>,
    pub read_dir: Option<String>,
}


impl SceneLayerReg {
    pub(crate) fn contains_path(&self) -> bool {
        self.write_dir.is_some() && self.read_dir.is_some() && self.current_save.is_some()
    }

    pub fn read(&self) -> SceneLayerReader {
        SceneLayerReader {
            layers: self.layers.read().unwrap(),
        }
    }
    // scene layers
    pub(crate) fn insert(&self, name: String, config: Box<dyn SceneConfig>) {
        self.layers.write().unwrap().insert(name, config);
    }

    pub fn write_dir(&self) -> &str {
        self.write_dir.as_ref().unwrap()
    }
    pub fn init_save(&mut self, name: String) {
        self.current_save = Some(name);
        let write_path = format!("assets/saves/{}", self.current_save.as_ref().unwrap());
        let read_path = format!("saves/{}", self.current_save.as_ref().unwrap());


        let path = Path::new(&write_path);
        if !path.exists() {
            info!("Creating directory: {}", &write_path);

            fs::create_dir_all(path).unwrap_or_else(|e| {
                error!("Failed to create directory {}: {}", &write_path, e);
            });
        }

        self.write_dir = Some(write_path);
        self.read_dir = Some(read_path);
    }
}


pub struct SceneLayerReader<'a> {
    layers: RwLockReadGuard<'a, HashMap<String, Box<dyn SceneConfig>>>,
}

impl SceneLayerReader<'_> {
    //noinspection RsExternalLinter
    pub fn get(&self, name: &str) -> Option<&Box<dyn SceneConfig>> {
        self.layers.get(name)
    }
}