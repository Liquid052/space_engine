use bevy::prelude::*;

use crate::prelude::*;
use crate::resources::*;

pub fn refresh(mut engine_building: ResMut<EngineBuilding>) {
    **engine_building = true;
}

pub fn move_state(
    mut next_state: ResMut<NextState<BuildingStates>>,
    engine_building: Res<EngineBuilding>
) {
    if **engine_building {
        next_state.set(BuildingStates::Finished);
    }
}

