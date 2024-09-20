use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct BodyTransferEvent {
    pub entity:     Entity,
    pub old_parent: Entity,
    pub new_parent: Entity,
    pub up:         bool,
}

impl BodyTransferEvent {
    pub fn new(entity: Entity, old_parent: Entity, new_parent: Entity, up: bool) -> Self {
        Self {
            entity,
            old_parent,
            new_parent,
            up,
        }
    }
}

#[derive(Event)]
pub(crate) struct VesselTransferEvent {
    pub entity:     Entity,
    pub old_parent: Entity,
    pub new_parent: Entity,
    pub up:         bool,
}

impl VesselTransferEvent {
    pub fn new(entity: Entity, old_parent: Entity, new_parent: Entity, up: bool) -> Self {
        Self {
            entity,
            old_parent,
            new_parent,
            up,
        }
    }
}
