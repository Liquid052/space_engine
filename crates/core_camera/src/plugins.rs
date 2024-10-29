use crate::prelude::*;
use bevy::prelude::*;

pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraManager::new())
            .register_type::<DefaultCameraLayer>()
            .register_type::<MainCamera>()
            .camera_manager()
            .config_layer(CamLayerConfig::new(DefaultCameraLayer)
                .clamp_zoom(0.1, 1000.0)
            );
    }
}