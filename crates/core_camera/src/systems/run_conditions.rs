// use std::any::TypeId;
// use bevy::prelude::*;
// use crate::prelude::*;

// pub(crate) fn cam_in_layer<T: Default + Component>(
//     cam_manager: Res<CameraManager>,
// ) -> bool {
//     TypeId::of::<T>() == cam_manager.current_layer()
// }