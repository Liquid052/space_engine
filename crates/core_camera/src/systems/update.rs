// use std::any::TypeId;
// use bevy::prelude::*;
// use crate::prelude::*;
//
// pub fn update_cam<T: Default + Component>(
//     mut cam_man: Res<CameraManager>,
//     mut commands: Commands,
// ) {
//     let t_id = TypeId::of::<T>();
//
//     let systems = cam_man.update.get(&t_id).expect("Cam Layer not initialized");
//
//     systems.iter()
//         .for_each(|system_id| commands.run_system(*system_id));
// }