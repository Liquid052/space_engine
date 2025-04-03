use bevy::prelude::*;

use crate::{prelude::*, systems::*};

pub(super) struct CamPlugin {
    pub enable_background: bool,
    pub cam_target: Option<String>,
}

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        // compos
        app.register_type::<FocusMode>()
            .register_type::<CamEnabled>();
            // systems

        match self.enable_background {
            false => { app.add_systems(Startup, spawn_cam); }
            true => { app.add_systems(Startup, spawn_cam_with_background);}
        }

       
        if self.cam_target.is_some() {
            let target = self.cam_target.clone().unwrap();


            app.add_systems(PostStartup, move |mut query: Query<&mut FocusMode, With<SpaceLayer>>| {
                query.iter_mut().for_each(|mut focus| {
                    *focus = FocusMode::Body(target.clone());
                });
            });
        }

       
    }
}
