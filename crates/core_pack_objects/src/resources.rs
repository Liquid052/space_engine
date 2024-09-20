use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use bevy::{prelude::*, utils::HashMap};
use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct Packer {
    pub transformer: Box<dyn PackTransformer>,
    pub required:    Vec<(String, TagType)>,
    pub optional:    Vec<(String, TagType)>,
    pub general_transformers: Vec<Arc<dyn GeneralTransformer>>,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct TaggedObjDB {
    map: Arc<RwLock<HashMap<String, Packer>>>,
    general_map: Arc<RwLock<HashMap<String, Arc<dyn GeneralTransformer>>>>,
}

impl TaggedObjDB {
    pub fn read(&self) -> Result<TaggedObjReader, TaggedObjError> {
        self.map.read()
            .map(|borrowed_map| TaggedObjReader { map: borrowed_map })
            .map_err(|_| TaggedObjError::LockError)
    }

    pub(crate) fn register(&self, builder: impl PackTransformer + 'static) {
        let transformer = Box::new(builder);
        let type_name = transformer.type_name().to_string();

        let required: Vec<(String, TagType)> = transformer.required()
            .iter()
            .map(|&(key, value)| (key.to_string(), value))
            .collect();
        let optional: Vec<(String, TagType)> = transformer.optional()
            .iter()
            .map(|&(key, value)| (key.to_string(), value))
            .collect();

        let mut write_g = self.map.write()
            .map_err(|_| TaggedObjError::LockError).unwrap();

        if write_g.contains_key(&type_name) {
            return;
        }

        let general_map = self.general_map.read().map_err(|_| TaggedObjError::LockError).unwrap();
        let general_transformers = required.iter()
            .chain(optional.iter())
            .filter_map(|(attr, _)| general_map.get(attr))
            .cloned()
            .collect();

        write_g.insert(type_name, Packer {
            transformer,
            required,
            optional,
            general_transformers,
        });
    }

    pub(crate) fn register_general(&self, builder: impl GeneralTransformer + 'static) {
        let attribute = builder.attribute().0.to_string();
        let builder = Arc::new(builder);

        let mut write_g = self.general_map.write()
            .map_err(|_| TaggedObjError::LockError)
            .unwrap();

        if write_g.contains_key(&attribute) {
            return;
        }

        write_g.insert(attribute.clone(), builder.clone());

        // Update existing Packers
        let mut map_write = self.map.write().map_err(|_| TaggedObjError::LockError)
            .unwrap();
        for packer in map_write.values_mut() {
            if packer.required.iter().any(|(attr, _)| attr == &attribute) ||
                packer.optional.iter().any(|(attr, _)| attr == &attribute) {
                packer.general_transformers.push(builder.clone());
            }
        }
    }
}

pub struct TaggedObjReader<'a> {
    map: RwLockReadGuard<'a, HashMap<String, Packer>>,
}

impl<'a> TaggedObjReader<'a> {
    pub fn pack(&self, mut ent_transform: EntityTransformer) {
        let type_name = ent_transform.main_entity_ref().get::<Pack>().unwrap()
            .type_name();

        let packer = self.map.get(type_name).unwrap();

        packer.transformer.pack(&mut ent_transform);

        packer.general_transformers.iter().for_each(|transformer| {

            transformer.pack(&mut ent_transform);
        });

        assert!(self.validate(packer, &ent_transform), "Tags component not valid - packing");
    }
    pub fn unpack(&self, mut ent_transform: EntityTransformer) {
        let type_name = ent_transform.main_entity_ref().get::<Pack>().unwrap()
            .type_name();

        let packer = self.map.get(type_name).unwrap();

        assert!(self.validate(packer, &ent_transform), "Tags component not valid - unpacking");

        packer.transformer.unpack(&mut ent_transform);

        packer.general_transformers.iter().for_each(|transformer| {
            let attribute = transformer.attribute().0;
            let contains_attribute = ent_transform.main_entity_ref().get_ref::<Pack>()
                .unwrap()
                .contains_key(attribute);

            if contains_attribute {
                transformer.unpack(&mut ent_transform);
            }
        });
    }

    fn validate(&self, packer: &Packer, ent_transformer: &EntityTransformer) -> bool {
        let tags = ent_transformer.main_entity_ref().get::<Pack>()
            .unwrap();

        let required = packer.required.iter().fold(true, |_, (attribute,tag)| {
            let attribute = attribute.as_str();

            let Some(tags_attribute) = tags.get(attribute) else {
                return false;
            };

            tags_attribute.matches(tag)
        });
        let optional = packer.optional.iter().fold(true, |_, (attribute,tag)| {
            let attribute = attribute.as_str();

            let Some(tags_attribute) = tags.get(attribute) else {
                return true;
            };

            tags_attribute.matches(tag)
        });

        required && optional
    }
}

#[derive(Debug)]
pub enum TaggedObjError {
    LockError,
    DuplicateBuilder(String),
}

impl fmt::Display for TaggedObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaggedObjError::LockError => write!(f, "Failed to acquire lock"),
            TaggedObjError::DuplicateBuilder(name) => write!(f, "TaggedObjDB already has a defined builder for {}", name),
        }
    }
}

impl Error for TaggedObjError {}
