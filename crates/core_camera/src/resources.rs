use crate::prelude::*;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use std::any::TypeId;

#[derive(Resource)]
pub struct CameraManager {
    pub(crate) layers: Vec<TypeId>,
    pub(crate) map: HashMap<TypeId, usize>,
    pub(crate) active_id: usize,

    pub(crate) update: HashMap<TypeId, Vec<SystemId>>,
    pub(crate) on_exit: HashMap<TypeId, Vec<SystemId>>,
    pub(crate) on_enter: HashMap<TypeId, Vec<SystemId>>,
    pub(crate) cam_styles: HashMap<TypeId, CameraStyle>,

    registered_layers: HashSet<TypeId>,
}

impl CameraManager {
    pub(crate) fn new() -> Self {
        let map = HashMap::default();

        Self {
            layers: vec![],
            map,
            active_id: 0,
            update: default(),
            on_exit: default(),
            on_enter: default(),
            cam_styles: default(),
            registered_layers: default(),
        }
    }

    pub fn current_layer(&self) -> TypeId {
        self.layers[self.active_id]
    }
    pub fn register_layer<T: 'static>(&mut self) {
        self.registered_layers.insert(TypeId::of::<T>());
    }
    pub fn add_layer<T: 'static>(&mut self) -> usize {
        let t_id = TypeId::of::<T>();
        self.layers.push(t_id);
        let len = self.layers.len() - 1;
        self.map.insert(t_id, len);

        len
    }


    pub fn get_layer_id<T: Resource>(&self) -> Option<usize> {
        self.map.get(&TypeId::of::<T>()).copied()
    }
}