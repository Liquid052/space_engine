use bevy::prelude::*;


#[derive(Resource, Debug)]
pub struct GalaxyGrid {
    pub entity: Entity,
    pub grid_size: IVec2,
    pub current_grid_pos: IVec2,
    pub mouse_pos: Vec2,
    pub active: bool,

}

impl GalaxyGrid {
    pub fn new(entity: Entity, grid_size: IVec2) -> Self {
        Self {
            entity,
            grid_size,
            current_grid_pos: IVec2::ZERO,
            mouse_pos: Vec2::ZERO,
            active: false,
        }
    }
}