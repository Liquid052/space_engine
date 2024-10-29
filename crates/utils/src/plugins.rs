use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod fullscreen_switch;
pub use fullscreen_switch::*;

pub struct UtilityPlugins;

impl PluginGroup for UtilityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FullscreenSwitchPlugin)
    }
}