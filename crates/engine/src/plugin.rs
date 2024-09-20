extern crate bevy;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::DefaultPlugins;
use bevy::prelude::Window;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use engine_core::prelude::CoreEnginePlugin;
use space::plugins::SpacePlugin;

// entry point
#[derive(Default)]
pub struct EnginePlugin {
    name: String,
    enable_space: bool,
}

impl EnginePlugin {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..default()
        }
    }
    pub fn enable_space(mut self) -> Self {
        self.enable_space = true;

        self
    }
}

impl PluginGroup for EnginePlugin {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins
                .set(WindowPlugin {
                primary_window: Some(Window {
                    title: self.name,
                    ..Default::default()
                }),
                ..Default::default()
            }))
            .add(CoreEnginePlugin::new());

        if self.enable_space {
            group = group.add(SpacePlugin {
                draw_enabled: true,
                cam_background_enabled: false,
                cam_target: None,
            });
        }

        group
    }
}
