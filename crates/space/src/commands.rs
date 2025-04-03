use crate::prelude::{FocusMode, SpaceLayer};
use crate::resources::StarSystem;
use bevy::ecs::world::Command;
use bevy::prelude::{info, Camera, Commands, With, World};


mod builders;

// export
pub use builders::*;

/// Commands for the space module
pub trait SpaceCommandsExt {
    /// creates a new space
    fn create_space(&mut self, name: &str);
    /// defines which celestial body the camera should follow
    fn space_cam_follow(&mut self, name: &str);
}

//noinspection RsExternalLinter
impl<'w, 's> SpaceCommandsExt for Commands<'w,'s> {
    fn create_space(&mut self, name: &str) {
        self.add(CreateSpaceCommand(name.into()))
    }
    fn space_cam_follow(&mut self, name: &str) {
        self.add(SpaceCamFollow(name.into()))
    }
}


// implementation
struct CreateSpaceCommand(pub String);
struct SpaceCamFollow(pub String);

impl Command for CreateSpaceCommand {
    fn apply(self, world: &mut World) {
        info!("Creating new space \"{}\"", self.0);

        world.resource_mut::<StarSystem>().system_name = self.0;
    }
}
impl Command for SpaceCamFollow {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<&mut FocusMode, (With<Camera>)>();
        
        let Ok(mut focus_mode) = query.get_single_mut(world) else {
            return;
        };

        *focus_mode = FocusMode::Body(self.0);
    }
}