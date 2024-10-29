use crate::prelude::CameraManager;
use bevy::ecs::world::Command;
use bevy::prelude::{error, Commands, Component, World};
use std::any::{type_name, TypeId};
use std::marker::PhantomData;

pub trait CameraManagementExt<'w, 's> {
    fn set_camera_layer<T: Component>(&mut self, marker: T) -> &mut Commands<'w, 's>;
}


impl<'w, 's> CameraManagementExt<'w, 's> for Commands<'w, 's> {
    fn set_camera_layer<T: Component>(&mut self, _marker: T) -> &mut Commands<'w, 's> {
        self.add(SetCameraLayer(PhantomData::<T>));

        self
    }
}

struct SetCameraLayer<T: Component>(PhantomData<T>);

impl<T: Component> Command for SetCameraLayer<T> {
    fn apply(self, world: &mut World) {
        let mut cam_manager = world.resource_mut::<CameraManager>();

        // if contains layer
        if !cam_manager.map.contains_key(&TypeId::of::<T>()) {
            error!("Layer {} does not exist", type_name::<T>());
            return;
        }

        let last_id = cam_manager.active_id;
        let last_type_id = cam_manager.layers[last_id];
        let id = cam_manager.map.get(&TypeId::of::<T>()).copied().unwrap();
        cam_manager.active_id = id;

        let exit_systems = cam_manager.on_exit.get(&last_type_id).cloned();
        let enter_systems = cam_manager.on_enter.get(&TypeId::of::<T>()).cloned();

        if let Some(systems) = exit_systems {
            systems.iter().for_each(|sys| {
                world.run_system(*sys).unwrap();
            });
        }
        if let Some(systems) = enter_systems {
            systems.iter().for_each(|sys| {
                world.run_system(*sys).unwrap();
            });
        }
    }
}