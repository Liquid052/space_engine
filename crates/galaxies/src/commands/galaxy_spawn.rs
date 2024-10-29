use crate::bundles::GalaxyBundle;
use crate::commands::galaxy::PopulateGalaxyGrid;
use crate::prelude::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use bevy::utils::default;
use engine_core::prelude::PosKey;

const ERR_MESSAGE: &str = "GalaxyCollection not loaded - perhaps missing enable_space() on EnginePlugin?";

pub struct SpawnGalaxy {
    pub(crate) name: String,
    pub(crate) _description: String,
    pub(crate) cell_size: i32,
    pub(crate) _id: u32,
}

impl Command for SpawnGalaxy {
    fn apply(self, world: &mut World) {
        if !world.contains_resource::<SpaceWorld>() {
            let mut commands = world.commands();

            commands.add(SpawnWorld);
            commands.add(self);

            return;
        }

        let galaxy_collection = world.get_resource::<GalaxyCollection>()
            .expect(ERR_MESSAGE)
            .clone();
        let map = galaxy_collection.map.clone_weak();
        let mask = galaxy_collection.mask.clone_weak();
        let images = world.resource::<Assets<Image>>();
        let map_size = images.get(&map).unwrap().size().as_ivec2();
        let mask_size = images.get(&mask).unwrap().size().as_ivec2();

        assert!(map_size.eq(&mask_size));

        let name = Name::new(self.name.clone());
        let galaxy = Galaxy {
            map: (map.clone_weak(), mask.clone_weak()),
            cell_size: self.cell_size,
            map_size,

            current_grid_pos: None,
            mouse_pos: None,
            children: default(),
            active_entity: None,
            selected_ent: None,
        };

        let entity = world.spawn(GalaxyBundle {
            name,
            galaxy,
            sprite_bundle: SpriteBundle {
                texture: map.clone_weak(),
                ..default()
            },
        }).id();

        world.resource_mut::<SpaceWorld>()
            .add_galaxy(PosKey::Pos(IVec2::ZERO), entity);

        world.commands()
            .add(PopulateGalaxyGrid);
    }
}