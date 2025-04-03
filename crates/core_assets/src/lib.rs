/*!

This crate provides an abstraction for asset management in Bevy applications, utilizing the 
[bevy_asset_loader](https://docs.rs/bevy_asset_loader/latest/bevy_asset_loader/) library. It is structured in four main stages: loading core assets, 
loading additional assets, content processing, and build state management. 

## Overview

The asset management system is designed to facilitate the loading and processing of game assets in a 
structured manner. It leverages Bevy's state machine capabilities to manage different loading states
and ensure that assets are loaded in the correct order.
*/

pub mod bevy_api;
pub mod states;

mod systems;
mod helpers;

#[doc(hidden)]
pub mod resources;
#[doc(hidden)]
pub mod plugins;
#[doc(hidden)]
pub mod assets;

#[doc(hidden)]
pub mod prelude;
