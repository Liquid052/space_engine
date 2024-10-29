use crate::prelude::Galaxy;
use bevy::prelude::*;

pub fn galaxy_active(galaxies: Query<&Galaxy>) -> bool {
    let Ok(galaxy) = galaxies.get_single() else {
        return false;
    };

    galaxy.is_active()
}