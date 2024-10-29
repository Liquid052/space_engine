use crate::prelude::Galaxy;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use space::prelude::{SpaceCommandsExt, Star};


pub trait GalaxyStarsSystemExt<'w, 's> {
    fn generate_star_system(&mut self, ent: Entity) -> &mut Commands<'w, 's>;
}

struct BuildStarSystem(pub Entity);


impl<'w, 's> GalaxyStarsSystemExt<'w, 's> for Commands<'w, 's> {
    fn generate_star_system(&mut self, ent: Entity) -> &mut Commands<'w, 's> {
        self.add(BuildStarSystem(ent));

        self
    }
}

impl Command for BuildStarSystem {
    fn apply(self, world: &mut World) {
        let mut galaxy = world.query::<&mut Galaxy>();
        let mut query = world.query::<&mut Star>();

        let Ok(star) = query.get_single(world).cloned() else {
            return;
        };

        galaxy.get_single_mut(world).unwrap()
            .set_active_entity(self.0);

        let mut commands = world.commands();

        commands.create_space(&star.name);
        commands.add(star);
    }
}