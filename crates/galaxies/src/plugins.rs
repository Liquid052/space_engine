use crate::prelude::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use engine_core::prelude::*;
use space::constants::SPACE_LAYER;

pub struct GalaxyPlugin;

impl Plugin for GalaxyPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ShapePlugin>() {
            app.add_plugins(ShapePlugin);
        }

        app.load_collection::<GalaxyCollection>()
            .register_type::<SpaceWorld>()
            .register_type::<Galaxy>()
            .add_systems(PreUpdate, (
                cam_controls,
                clamp_camera_values,
                update_grid
                    .run_if(one_with_component::<Galaxy>)
            )
                .chain(),
            )
            .camera_manager()
            .config_layer(CamLayerConfig::new(GalaxyLayer)
                .render_layer(SPACE_LAYER)
                .clamp_zoom(0.1, 1000.0)
                .depth(100.0)
            );
    }
}
