use bevy::prelude::*;

use crate::{components::Space, resources::OrbitBuffer};

pub fn cam_scale_change(
    cams: Query<&OrthographicProjection, (Changed<OrthographicProjection>, With<Space>)>,
) -> bool {
    !cams.is_empty()
}

pub fn orbit_buff_not_empty(buff: Res<OrbitBuffer>) -> bool { !buff.vec.is_empty() }
