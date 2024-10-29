use crate::prelude::{AOSBody, SpaceTimeScale};
use crate::systems::aos::*;
use crate::systems::SpaceSystemSet;
use bevy::prelude::*;

// Arrays of struct version
pub struct AOSSpacePlugin;

impl Plugin for AOSSpacePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AOSBody>()
            // orbit
            .insert_resource(SpaceTimeScale(3.0))
            .register_type::<SpaceTimeScale>()
            .add_systems(
                Update,
                (
                    update_period_aos,
                    update_epochs_aos,
                    update_orbits_aos,
                ).in_set(SpaceSystemSet::Physics),
            )
            .add_systems(Update, map_orbit_pos_aos.after(SpaceSystemSet::Physics));
    }
}

