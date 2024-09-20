use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;

use cam::*;
use draw::*;
use physics::*;

use crate::{
    events::{BodyTransferEvent, VesselTransferEvent},
    prelude::CamTargetEv,
    resources::SpaceMap,
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
    pub cam_background_enabled: bool,
    pub cam_target: Option<String>,
}

impl Default for SpacePlugin {
    fn default() -> Self { Self::new() }
}

impl SpacePlugin {
    pub fn new() -> Self { Self { draw_enabled: true, cam_background_enabled: false, cam_target: None } }

    pub fn disable_draw(mut self) -> Self {
        self.draw_enabled = false;
        self
    }
}
impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        config_set(app);

        // plugins
        app.add_plugins(PhysicsPlugin)
            .add_plugins(CamPlugin {
                enable_background: self.cam_background_enabled,
                cam_target: self.cam_target.clone(),
            });

        if self.draw_enabled {
            app.add_plugins(DrawPlugin);
        }

        // diagnostics
        app.add_plugins(PerfUiPlugin)
            // resources
            .init_resource::<SpaceMap>()
            // events
            .add_event::<BodyTransferEvent>()
            .add_event::<VesselTransferEvent>()
            .add_event::<CamTargetEv>()
            // systems
            .add_systems(
                PostUpdate,
                    update_belts.in_set(SpaceSystemSet::UpdateDraw),
            )
            .add_systems(Last, cleanup_restrictions);
    }
}
