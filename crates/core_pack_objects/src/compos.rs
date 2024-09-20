use bevy::ecs::component::{ComponentHooks, StorageType};
use crate::prelude::{TaggedObjDB, EntityTransformer};
use bevy::ecs::world::DeferredWorld;
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::utils::HashMap;
use bevy::prelude::*;
use std::borrow::Cow;
use bevy::ecs::entity::MapEntities;

mod from_impls;


#[derive(Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct UnpackOnLoad;

impl Component for UnpackOnLoad {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world: DeferredWorld, target_ent, _| {
            if world.entity(target_ent).contains::<Unpacked>() {
                return;
            }

            world.commands().entity(target_ent)
                .insert(Unpacked);
        });
    }
}


#[derive(Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Unpacked;

#[derive(Reflect, Component, Default, Clone, Copy)]
#[reflect(Component)]
pub struct FromScene;

impl Component for Unpacked {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|world, target_ent, _| {
            if !world.entity(target_ent).contains::<Pack>() {
                return;
            }

            let obj_db: TaggedObjDB = world.resource::<TaggedObjDB>().clone();

            let ent_transformer = EntityTransformer::new(target_ent, world);

            {
                let read_lock = obj_db.read().unwrap();
                read_lock.unpack(ent_transformer);
            }
        });

        hooks.on_remove(|world, target_ent, _| {
            if !world.entity(target_ent).contains::<Pack>() {
                return;
            }

            let obj_db: TaggedObjDB = world.resource::<TaggedObjDB>().clone();

            let ent_transformer = EntityTransformer::new(target_ent, world);

            {
                let read_lock = obj_db.read().unwrap();
                read_lock.pack(ent_transformer);
            }

        });
    }
}

#[derive(Reflect, Clone, Component, Default)]
#[reflect(Component, MapEntities)]
pub struct Pack {
    type_name: Cow<'static, str>,
    tags:      HashMap<Cow<'static, str>, PackTag>,
}

impl MapEntities for Pack {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.tags.iter_mut().for_each(|(_, tag)| {
            tag.map_entities(entity_mapper);
        });
    }
}

impl Pack {
    // constructors
    pub fn new(type_name: impl Into<Cow<'static, str>>) -> Self {
        let type_name = type_name.into();

        assert!(!type_name.is_empty());

        Self {
            type_name,
            tags: Default::default(),
        }
    }

    pub fn valid(&self) -> bool {
        !self.type_name.is_empty()
    }

    pub fn with(mut self, key: impl Into<Cow<'static, str>>, value: PackTag) -> Self {
        let name = key.into();

        assert!(!name.is_empty());
        assert!(!self.tags.contains_key(&name));

        self.tags.insert(name, value);

        self
    }
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
    pub fn get_type(&self) -> Option<&str> {
        let tag = self.get("type")?;
        if let PackTag::String(type_name) = tag {
            Some(type_name)
        } else {
            None
        }
    }

    pub fn take(&mut self, key: &str) -> Option<PackTag> {
        self.tags.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&PackTag> {
        self.tags.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.tags.contains_key(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<PackTag> {
        self.tags.remove(key)
    }

    pub fn insert(&mut self, key: impl Into<Cow<'static, str>>, value: PackTag) -> &mut Self {
        self.tags.insert(key.into(), value);

        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Cow<'_, str>, &PackTag)> {
        self.tags.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Cow<'_, str>, &mut PackTag)> {
        self.tags.iter_mut()
    }



}

#[derive(Reflect, Component, Debug, Clone)]
#[reflect(no_field_bounds, MapEntities)]
pub enum PackTag {
    // primitive types
    Bool(bool),
    // integers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    // unsigned integers
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    // floating
    F32(f32),
    F64(f64),
    // String variants
    CowStr(Cow<'static, str>),
    String(String),
    // optional
    OptI32(Option<i32>),
    OptI64(Option<i64>),
    OptU32(Option<u32>),
    OptU64(Option<u64>),
    OptF32(Option<f32>),
    OptF64(Option<f64>),
    OptBool(Option<bool>),
    OptString(Option<String>),
    Entity(Entity),
    // custom
    Vec(Vec<PackTag>),
    Map(HashMap<String, PackTag>)
}


impl MapEntities for PackTag {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        match self {
            PackTag::Entity(ent) => *ent = mapper.map_entity(*ent),
            PackTag::Vec(vec) => vec.iter_mut()
                .for_each(|tag|
                tag.map_entities(mapper)
                ),
            PackTag::Map(map) => map.iter_mut()
                .for_each(|(_, tag)|
                tag.map_entities(mapper)
                ),
            _ => {}
        }
    }
}

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq)]
#[reflect(no_field_bounds)]
pub enum TagType {
    // primitive types
    Bool,
    // integers
    U8,
    U16,
    U32,
    U64,
    // signed integers
    I8,
    I16,
    I32,
    I64,
    // floating
    F32,
    F64,
    // String variants
    CowStr,
    String,
    // optional
    OptI32,
    OptI64,
    OptU32,
    OptU64,
    OptF32,
    OptF64,
    OptBool,
    OptString,
    Entity,
    // custom
    Vec,
    Map
}

impl TagType {
    pub fn matches(&self, tag_type: &PackTag) -> bool {
        tag_type.matches(self)
    }
}

impl PackTag {
    pub fn matches(&self, tag_type: &TagType) -> bool {
        match (self, tag_type) {
            (PackTag::Bool(_), TagType::Bool) => true,
            (PackTag::U8(_), TagType::U8) => true,
            (PackTag::U16(_), TagType::U16) => true,
            (PackTag::U32(_), TagType::U32) => true,
            (PackTag::U64(_), TagType::U64) => true,
            (PackTag::I8(_), TagType::I8) => true,
            (PackTag::I16(_), TagType::I16) => true,
            (PackTag::I32(_), TagType::I32) => true,
            (PackTag::I64(_), TagType::I64) => true,
            (PackTag::F32(_), TagType::F32) => true,
            (PackTag::F64(_), TagType::F64) => true,
            (PackTag::CowStr(_), TagType::CowStr) => true,
            (PackTag::String(_), TagType::String) => true,
            (PackTag::OptI32(_), TagType::OptI32) => true,
            (PackTag::OptI64(_), TagType::OptI64) => true,
            (PackTag::OptU32(_), TagType::OptU32) => true,
            (PackTag::OptU64(_), TagType::OptU64) => true,
            (PackTag::OptF32(_), TagType::OptF32) => true,
            (PackTag::OptF64(_), TagType::OptF64) => true,
            (PackTag::Entity(_), TagType::Entity) => true,
            (PackTag::OptBool(_), TagType::OptBool) => true,
            (PackTag::OptString(_), TagType::OptString) => true,
            (PackTag::Vec(_), TagType::Vec) => true,
            (PackTag::Map(_), TagType::Map) => true,
            _ => false,
        }
    }
}