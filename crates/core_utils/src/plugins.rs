use bevy::prelude::*;
use crate::prelude::*;

pub struct CoreUtilsPlugin;

impl Plugin for CoreUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Active>()
            .register_type::<Enabled>()
            .register_type::<DefaultMarker>();
    }
}