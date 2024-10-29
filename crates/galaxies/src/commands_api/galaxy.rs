use crate::builders::galaxy::GalaxyBuilder;
use bevy::prelude::{default, Commands};

pub trait GalaxySpaceExt<'w, 's> {
    fn galaxy_builder(self, name: impl Into<String>) -> GalaxyBuilder<'w, 's>;
}

impl<'w, 's> GalaxySpaceExt<'w, 's> for Commands<'w, 's> {
    fn galaxy_builder(self, name: impl Into<String>) -> GalaxyBuilder<'w, 's> {
        GalaxyBuilder {
            commands: self,
            name: name.into(),
            description: "".into(),
            cell_size: 16,
            pos: default(),
            id: 0,
        }
    }
}