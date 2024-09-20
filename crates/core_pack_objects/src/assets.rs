#![allow(unused)]
use bevy::asset::Asset;
use bevy::prelude::TypePath;
use bevy::reflect::erased_serde::__private::serde::Deserialize;

//noinspection ALL
#[derive(Asset, Deserialize, Debug, TypePath, Clone)]
pub struct ItemAsset {
    pub(crate) name:        String,
    pub(crate) db_name:     String,
    pub(crate) description: Option<String>,
    pub(crate) version:     String,

    pub(crate) db_prefix:  String,
    pub(crate) asset_path: String,
    pub(crate) dependencies: Vec<String>,
    pub(crate) untyped:      Vec<String>,
}