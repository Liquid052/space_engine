use crate::builders::StarBuilder;
use bevy::prelude::*;
use space::prelude::Star;

pub trait GalaxyStarsExt<'w, 's> {
    fn add_star(self, grid_pos: IVec2, name: impl Into<String>, star: Star) -> StarBuilder<'w, 's>;
}

impl<'w, 's> GalaxyStarsExt<'w, 's> for Commands<'w, 's>
where
    'w: 's,
{
    fn add_star(self, grid_pos: IVec2, name: impl Into<String>, star: Star) -> StarBuilder<'w, 's> {
        StarBuilder {
            commands: self,
            star,
            name: name.into(),
            grid_pos,
            cursor_pos: default(),
        }
    }
}