use crate::commands::SpawnGalaxy;
use bevy::math::IVec2;
use bevy::prelude::Commands;

pub struct GalaxyBuilder<'w, 's> {
    pub(crate) commands: Commands<'w, 's>,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) cell_size: i32,
    pub(crate) id: u32,
    pub(crate) pos: IVec2,
}

impl<'w, 's> GalaxyBuilder<'w, 's> {
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    pub fn cell_size(mut self, length: i32) -> Self {
        self.cell_size = length;
        self
    }
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
    pub fn pos(mut self, pos: impl Into<IVec2>) -> Self {
        self.pos = pos.into();
        self
    }
    pub fn build(self) -> Commands<'w, 's> {
        let mut commands = self.commands;

        commands.add(SpawnGalaxy {
            name: self.name,
            _description: self.description,
            cell_size: self.cell_size,
            _id: self.id,
        });

        commands
    }
}