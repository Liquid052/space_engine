#![ allow(unused)]
use std::sync::Arc;

use bevy::asset::{Asset, AssetServer, Assets, Handle, LoadedFolder, UntypedHandle};
use bevy::prelude::{Deref, DerefMut, FromWorld, Resource, World};
use bevy::utils::HashMap;

use crate::assets::ModInfo;
use crate::prelude::{ContentCache, ModsCache};

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct Mods(pub Arc<HashMap<String, ModInfo>>);

type Category = String;
type ItemName = String;

impl FromWorld for Mods {
    fn from_world(world: &mut World) -> Self {
        let cache = world.resource::<ModsCache>();
        let assets  = world.resource::<Assets<ModInfo>>();

        let map = cache.iter()
            .map(|handle| assets.get(handle).unwrap())
            .map(|mod_info| (mod_info.db_name.clone(), mod_info.clone()))
            .collect();

        Self(Arc::new(map))
    }
}

//noinspection RsExternalLinter
#[derive(Resource, Debug)]
pub struct AssetDatabase {
    pub(crate) items:      HashMap<ItemName, UntypedHandle>,
    pub(crate) categories: HashMap<Category, Vec<(ItemName, UntypedHandle)>>,

    pub(crate) assets: HashMap<String, UntypedHandle>,
}

impl FromWorld for AssetDatabase {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let cache      = world.resource::<ContentCache>();
        let folders= world.resource::<Assets<LoadedFolder>>();

        let (items, categories) = cache.handles.iter()
            .flat_map(|(db_prefix, category, handle)| {
                folders.get(handle)
                    .unwrap()
                    .handles
                    .iter()
                    .map(|untyped| (db_prefix.as_str(), category.as_str(), untyped))
            })
            .map(|(db_prefix, category, untyped)| {
                let id = untyped.id();
                let asset_path = asset_server.get_path(id).unwrap();
                let path = asset_path.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let asset_name = format!("{}:{}", db_prefix, file_name);

                (asset_name.clone(), category, untyped.clone())
            })
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut items, mut categories), (asset_name, category, untyped)| {

                    items.insert(asset_name.clone(), untyped.clone());
                    categories
                        .entry(category.to_string())
                        .or_insert_with(Vec::new)
                        .push((asset_name, untyped));
                    (items, categories)
                }
            );

        let assets = cache.untyped.iter()
            .flat_map(|(prefix,handle)| {
                folders.get(handle)
                    .unwrap()
                    .handles
                    .iter()
                    .map(|untyped| (prefix.as_str(), untyped))
            })
            .map(|(db_prefix, untyped)| {
                let id = untyped.id();

                let asset_path = asset_server.get_path(id).unwrap();
                let path                 = asset_path.path();

                let file_name = path.file_name().unwrap().to_str().unwrap();
                let asset_name = format!("{}:{}", db_prefix, file_name);

                (asset_name, untyped.clone())
            })
            .collect();

        Self {
            assets,
            categories,
            items,
        }
    }
}

impl AssetDatabase {
    pub fn asset_untyped(&self, name: &str) -> Option<&UntypedHandle> {
        self.assets.get(name)
    }
    //noinspection ALL
    pub fn asset<T: Asset>(&self, name: &str) -> Option<Handle<T>> {
        self.assets.get(name)
            .and_then(|handle| {
                Some(handle.clone_weak().typed::<T>())
            })
    }
}