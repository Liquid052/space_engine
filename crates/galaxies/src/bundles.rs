use crate::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub(crate) struct GalaxyBundle {
    pub name: Name,
    pub galaxy: Galaxy,
    pub sprite_bundle: SpriteBundle,
}