use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy::utils::HashMap;
use engine_core::prelude::*;

#[derive(Reflect, Component, Clone, Eq, PartialEq, Hash, Debug)]
pub struct WorldManager;

#[derive(Reflect, Component, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GalaxyLayer;

impl SceneLayerHandler for GalaxyLayer {
    fn save(&self, _: &SceneNode, _: &Config, _: &mut World) {}
    fn load(&self, _: &SceneNode, _: &Config, _: &mut World) {}
}
impl SceneLayerHandler for WorldManager {
    fn save(&self, _: &SceneNode, _: &Config, _: &mut World) {}
    fn load(&self, _: &SceneNode, _: &Config, _: &mut World) {}
}


// aliases
type MapHandle = Handle<Image>;
type MaskHandle = Handle<Image>;


#[derive(Reflect, Component, Debug, Clone)]
#[reflect(Component)]
pub struct Galaxy {
    pub(crate) map_size: IVec2,
    pub(crate) cell_size: i32,

    pub(crate) map: (MapHandle, MaskHandle),
    pub(crate) children: HashMap<PosKey, Entity>,

    pub(crate) active_entity: Option<Entity>,
    pub(crate) selected_ent: Option<Entity>,
    pub(crate) current_grid_pos: Option<IVec2>,
    pub(crate) mouse_pos: Option<Vec2>,
}

impl MapEntities for Galaxy {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.children.iter_mut()
            .for_each(|(_, ent)| *ent = entity_mapper.map_entity(*ent));
    }
}

impl Galaxy {
    pub fn new(grid_cell_size: i32, map_size: IVec2, map: Handle<Image>, mask: Handle<Image>) -> Self {
        Self {
            cell_size: grid_cell_size,
            map_size,
            map: (map, mask),

            children: default(),
            active_entity: None,
            selected_ent: None,
            current_grid_pos: None,
            mouse_pos: None,
        }
    }
    pub fn active_entity(&self) -> Option<Entity> {
        self.active_entity
    }
    pub fn selected_entity(&self) -> Option<Entity> {
        self.selected_ent
    }
    pub fn set_active_entity(&mut self, entity: Entity) {
        self.active_entity = Some(entity);
    }
    pub fn set_selected_entity(&mut self, entity: Entity) {
        self.selected_ent = Some(entity);
    }
    pub fn get_child(&self, key: &IVec2) -> Option<Entity> {
        self.children.get(&PosKey::Pos(*key)).copied()
    }

    pub fn is_occupied(&self, key: &IVec2) -> bool {
        self.children.contains_key(&PosKey::Pos(*key))
    }
    pub fn add_child(&mut self, key: PosKey, entity: Entity) {
        self.children.insert(key, entity);
    }

    pub fn map_size(&self) -> IVec2 {
        self.map_size
    }
    pub fn cell_size(&self) -> i32 {
        self.cell_size
    }

    pub fn mouse_pos(&self) -> Option<Vec2> {
        self.mouse_pos
    }
    pub fn grid_pos(&self) -> Option<IVec2> {
        self.current_grid_pos
    }
    pub fn is_active(&self) -> bool {
        self.current_grid_pos.is_some() && self.mouse_pos.is_some()
    }
}