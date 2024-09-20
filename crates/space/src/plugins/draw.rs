use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

use crate::{resources::*, systems::*};

pub(super) struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin).init_resource::<OrbitBuffer>();

        app.add_systems(
            PostUpdate,
            (install_draw, (install_orbit_outline, install_soi_outline))
                .chain()
                .in_set(SpaceSystemSet::InstallDraw),
        )
        // orbit update
        .add_systems(
            PostUpdate,
            (
                update_strokes,
                update_orbit_shape,
                update_requested_orbits.run_if(orbit_buff_not_empty),
            )
                .chain()
                .in_set(SpaceSystemSet::UpdateDraw)
                .run_if(cam_scale_change),
        )
        // soi
        .add_systems(
            PostUpdate,
            update_soi_outline
                .in_set(SpaceSystemSet::UpdateDraw)
                .after(install_soi_outline),
        );
    }
}
