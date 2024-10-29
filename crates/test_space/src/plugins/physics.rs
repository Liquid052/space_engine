use crate::{
    components::{Body, Keplerian, Orbit, RefFrame, SpaceDepth, SpacePos, StateVec},
    resources::SpaceTimeScale,
    systems::{
        update_changes::update_period,
        *,
    },
};
use bevy::prelude::*;

pub(super) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // resources
        app.insert_resource(SpaceTimeScale(3.0))
            // reflections
            .register_type::<SpaceDepth>()
            .register_type::<SpaceTimeScale>()
            .register_type::<Keplerian>()
            .register_type::<StateVec>()
            .register_type::<SpacePos>()
            .register_type::<Body>()
            .register_type::<Orbit>()
            .register_type::<RefFrame>()
            // systems
            // orbit
            .add_systems(
                Update,
                (
                    update_period,
                    update_epochs,
                    update_orbits,
                ).in_set(SpaceSystemSet::Physics),
            )
            .add_systems(Update, map_orbit_pos.after(SpaceSystemSet::Physics));
    }
}

