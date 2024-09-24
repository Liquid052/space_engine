use bevy::asset::{Assets, AssetServer, Handle, LoadedFolder, UntypedHandle};
use bevy::log::info;
use bevy::prelude::{Deref, DerefMut, Resource, World};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;
use crate::resources::*;
use crate::resources::assets::Mods;

#[derive(Resource, Default, DerefMut, Deref)]
pub struct EngineBuilding(pub bool);

#[derive(AssetCollection, Clone, Resource)]
pub struct ModPaths {
    #[asset(path = "mods/paths.mod_index.ron")]
    pub(crate) handle: Handle<ModIndex>,
}

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct ModsCache(pub(crate) Vec<Handle<ModInfo>>);

impl AssetCollection for ModsCache {
    fn create(world: &mut World) -> Self {
        let asset_server      = world.resource::<AssetServer>();
        let index_handle = world.resource::<ModPaths>().clone().handle.clone_weak();
        let assets      = world.resource::<Assets<ModIndex>>();

        let path = assets.get(&index_handle).unwrap().clone();
        let mods = path.paths.into_iter()
            .map(|path| {
                asset_server.load::<ModInfo>(path)
            })
            .collect();

        Self(mods)
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        let asset_server = world.resource::<AssetServer>().clone();
        let index_handle = world.resource::<ModPaths>().clone().handle.clone_weak();

        let assets = world.resource::<Assets<ModIndex>>();

        let Some(index) = assets.get(&index_handle).cloned() else {
            return vec![];
        };

        let paths = index.paths.into_iter()
            .map(|path| {
                asset_server.load::<ModInfo>(path).untyped()
            })
            .collect();

        paths
    }
}





#[derive(Resource, Debug, Default, Clone)]
pub struct ContentCache {
    pub(crate) handles: Vec<(DbPrefix, Category, Handle<LoadedFolder>)>,
    pub(crate) untyped: Vec<(DbPrefix, Handle<LoadedFolder>)>
}

impl AssetCollection for ContentCache {
    fn create(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>().clone();
        let mods = world.resource::<Mods>();
        let mut handles = Vec::new();
        let mut untyped = Vec::new();

        mods.iter()
            .map(|(_,mod_info)| mod_info)
            .for_each(|mod_info| {
                let asset_path = mod_info.asset_path.as_str();
                let db_name = mod_info.db_name.as_str();

                // handles
                mod_info.dependencies.iter()
                    .for_each(|category| {
                        let path = format!("mods/{}/{}", asset_path, category);

                        info!("{}", path);

                        handles.push((
                            db_name.to_string(),
                            category.clone(),
                            asset_server.load_folder(path)
                        ))
                    });

                // untyped handles
                mod_info.untyped.iter()
                    .for_each(|path| {
                        let path = format!("mods/{}/{}", asset_path, path);

                        untyped.push((mod_info.db_prefix.clone(),asset_server.load_folder(path)));
                    });
            });

        Self {
            handles,
            untyped,
        }
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        let asset_server = world.resource::<AssetServer>().clone();
        let mods = world.resource::<Mods>();

        let mut handles = Vec::new();

        mods.iter()
            .map(|(_,mod_info)| mod_info)
            .for_each(|mod_info| {
                let asset_path = mod_info.asset_path.as_str();

                // handles
                mod_info.dependencies.iter()
                    .for_each(|category| {
                        let path = format!("mods/{}/{}", asset_path, category);

                        handles.push(asset_server.load_folder(path));
                    });

                // untyped handles
                mod_info.untyped.iter()
                    .for_each(|path| {
                        let path = format!("mods/{}/{}", asset_path, path);

                        handles.push(asset_server.load_folder(path));
                    });
            });


        handles.into_iter()
            .map(|handle| handle.untyped())
            .collect()
    }
}
