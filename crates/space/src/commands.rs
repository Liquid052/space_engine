use bevy::ecs::world::Command;
use bevy::prelude::{Camera, Commands, info, With, World};
use crate::prelude::{FocusMode, Space};
use crate::resources::SpaceMap;


mod builders;

// export
pub use builders::*;

// api
pub trait SpaceCommandsExt {
    fn create_space(&mut self, name: &str);
    fn space_cam_follow(&mut self, name: &str);
}

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
        
        world.resource_mut::<SpaceMap>().name = self.0;
    }
}
impl Command for SpaceCamFollow {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<&mut FocusMode, (With<Camera>, With<Space>)>();
        
        let Ok(mut focus_mode) = query.get_single_mut(world) else {
            return;
        };

        *focus_mode = FocusMode::Body(self.0);
    }
}