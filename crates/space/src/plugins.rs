use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

use cam::*;
use draw::*;
use engine_core::prelude::*;
use physics::*;

use crate::prelude::{SpaceLayer, Star, SPACE_LAYER};
use crate::{
    events::{BodyTransferEvent, VesselTransferEvent},
    prelude::CamTargetEv,
    resources::StarSystem,
    systems::{
        belts::update_belts,
        *,
    },
};

mod cam;
mod draw;
mod physics;

pub struct SpacePlugin {
    pub draw_enabled: bool,
    pub camera_enabled: bool,
    pub cam_background_enabled: bool,
    pub cam_target: Option<String>,
}

impl Default for SpacePlugin {
    fn default() -> Self { Self::new() }
}

impl SpacePlugin {
    pub fn new() -> Self { Self { draw_enabled: true, camera_enabled: false, cam_background_enabled: false, cam_target: None } }

    pub fn disable_draw(mut self) -> Self {
        self.draw_enabled = false;
        self
    }
}
impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        config_set(app);

        // plugins
        app.add_plugins(PhysicsPlugin);

        if self.camera_enabled {
            app.add_plugins(CamPlugin {
                enable_background: self.cam_background_enabled,
                cam_target: self.cam_target.clone(),
            });
        }

        if self.draw_enabled {
            app.add_plugins(DrawPlugin);
        }

        // diagnostics
        app.add_plugins(PerfUiPlugin)
            .register_type::<Star>()
            // resources
            .init_resource::<StarSystem>()
            // events
            .add_event::<BodyTransferEvent>()
            .add_event::<VesselTransferEvent>()
            .add_event::<CamTargetEv>()
            // systems
            .add_systems(
                PostUpdate,
                    update_belts.in_set(SpaceSystemSet::UpdateDraw),
            )
            .add_systems(Last, cleanup_restrictions)
            .camera_manager()
            .config_layer(CamLayerConfig::new(SpaceLayer)
                .clamp_zoom(0.1, 1000.0)
                .enable_hdr()
                .tone_mapping(Tonemapping::AcesFitted)
                .render_layer(SPACE_LAYER)
                .depth(100.0)
            )
            .app();
    }
}
