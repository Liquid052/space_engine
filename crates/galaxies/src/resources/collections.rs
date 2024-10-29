use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource, Clone)]
pub struct GalaxyCollection {
    #[asset(path = "core/space/galaxy_text.png")]
    #[asset(image(sampler = nearest))]
    pub map: Handle<Image>,

    #[asset(path = "core/space/galaxy_mask.png")]
    #[asset(image(sampler = nearest))]
    pub mask: Handle<Image>,
}