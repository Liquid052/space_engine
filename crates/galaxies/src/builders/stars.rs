use crate::prelude::Galaxy;
use bevy::ecs::world::Command;
use bevy::math::IVec2;
use bevy::prelude::*;
use engine_core::prelude::*;
use space::prelude::Star;

pub struct StarBuilder<'w, 's> {
    pub(crate) commands: Commands<'w, 's>,
    pub(crate) star: Star,
    pub(crate) name: String,

    pub(crate) grid_pos: IVec2,
    pub(crate) cursor_pos: Option<Vec2>,
}
struct Inner {
    pub(crate) star: Star,
    pub(crate) name: String,

    pub(crate) grid_pos: IVec2,
    pub(crate) cursor_pos: Option<Vec2>,
}


impl<'w, 's> StarBuilder<'w, 's> {
    pub fn cursor_pos(mut self, pos: Vec2) -> Self {
        self.cursor_pos = Some(pos.into());
        self
    }
    pub fn build(self) -> Commands<'w, 's> {
        let mut commands = self.commands;

        commands.add(Inner {
            star: self.star,
            name: self.name,
            grid_pos: self.grid_pos,
            cursor_pos: self.cursor_pos,
        });

        commands
    }
}
impl Command for Inner {
    fn apply(self, world: &mut World) {
        let mut query = world.query::<&Galaxy>();
        let galaxy = query.get_single(world).unwrap();

        if galaxy.is_occupied(&self.grid_pos) {
            return;
        }

        let cell_size = galaxy.cell_size() as f32 / 2.0;
        let grid_start = galaxy.map_size().as_vec2() / 2.0;

        let grid_pos = self.grid_pos;
        let cursor_pos = self.cursor_pos.unwrap();

        let offset = Vec3::new(cursor_pos.x - grid_start.x + cell_size, cursor_pos.y - grid_start.y + cell_size, 8.0);
        info!("Spawn star [{}]", offset);

        let asset_server = world.resource::<AssetServer>().clone();
        let id = world.spawn((SpriteBundle {
            transform: Transform::default().with_translation(offset),
            texture: asset_server.load("core/space/star_icon.png"),
            ..default()
        }, Name::new(self.name), self.star
        ))
            .id();

        let mut q = world.query::<&mut Galaxy>();
        let mut galaxy = q.get_single_mut(world).unwrap();

        galaxy.children.insert(PosKey::Pos(grid_pos), id);
    }
}