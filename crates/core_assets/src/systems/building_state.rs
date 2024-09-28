use bevy::prelude::*;

use crate::prelude::*;
use crate::resources::*;

pub fn refresh(mut engine_building: ResMut<EngineBuildFinished>) {
    **engine_building = true;
}

pub fn move_state(
    mut next_state: ResMut<NextState<BuildingStates>>,
    engine_build_finished: Res<EngineBuildFinished>
) {
    if **engine_build_finished {
        next_state.set(BuildingStates::Finished);
    }
}

