#![allow(dead_code)]
use bevy::prelude::*;
use serde::Deserialize;

//noinspection RsExternalLinter
#[derive(Asset, Deserialize, Debug, TypePath, Clone)]
pub struct ModInfo {
    pub(crate) name:        String,
    pub(crate) db_name:     String,
    pub(crate) description: Option<String>,
    pub(crate) version:     String,

    pub(crate) db_prefix:    String,
    pub(crate) asset_path:   String,
    pub(crate) dependencies: Vec<String>,
    pub(crate) untyped:      Vec<String>,
}

#[derive(Asset, Deserialize, Debug, TypePath, Clone)]
pub struct ModIndex {
    pub paths: Vec<String>,
}

