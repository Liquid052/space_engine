use crate::prelude::CameraManager;
use crate::systems::{add_to_cam, remove_from_cam, update_style};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::any::{type_name, TypeId};

pub trait CameraManagerBuildExt {
    fn camera_manager(&mut self) -> CameraBuilder;
}

pub struct CameraBuilder<'w> {
    app: &'w mut App,

    // configs: Vec<(TypeId, Box::<dyn ApplyConfig>)>,
}

// trait ApplyConfig: Send + Sync{
//     fn apply_config(&mut self, config: &mut App);
// }

#[derive(Default, Clone)]
pub struct CamLayerConfig<T> {
    _marker: T,

    style: Option<CameraStyle>,
}

#[derive(Default, Clone)]
pub struct CameraStyle {
    // update
    pub clamp_zoom: Option<(f32, f32)>,
    // on-enter
    pub tone_mapping: Option<Tonemapping>,
    pub render_layer: Option<RenderLayers>,
    pub bloom: Option<BloomSettings>,
    pub hdr: bool,
    pub depth: Option<f32>,
}


impl<T: Default + Component> CamLayerConfig<T> {
    pub fn new(marker: T) -> Self {
        Self {
            _marker: marker,
            style: None,
        }
    }

    fn get_or_create(&mut self) -> &mut CameraStyle {
        if self.style.is_none() {
            self.style = Some(CameraStyle::default());
        }

        self.style.as_mut().unwrap()
    }
    pub fn tone_mapping(mut self, tonemapping: Tonemapping) -> Self {
        self.get_or_create().tone_mapping = Some(tonemapping);

        self
    }
    pub fn bloom(mut self, bloom: BloomSettings) -> Self {
        self.get_or_create().bloom = Some(bloom);

        self
    }
    pub fn clamp_zoom(mut self, min: f32, max: f32) -> Self {
        self.get_or_create().clamp_zoom = Some((min, max));

        self
    }
    pub fn render_layer(mut self, layer: RenderLayers) -> Self {
        self.get_or_create().render_layer = Some(layer);

        self
    }
    pub fn depth(mut self, depth: f32) -> Self {
        self.get_or_create().depth = Some(depth);

        self
    }
    pub fn enable_hdr(mut self) -> Self {
        self.get_or_create().hdr = true;

        self
    }

    //noinspection ALL
    //noinspection RsExternalLinter
    //noinspection ALL
    fn apply_config(&mut self, config: &mut App) {
        let world = config.world_mut();

        info!("Registering layer: {:?}", type_name::<T>());
        let update_systems = vec![];
        let mut add_systems = vec![];
        let mut rem_systems = vec![];

        let cloned = self.style.clone();

        if let Some(_style) = &cloned {
            add_systems.push(world.register_system(update_style::<T>));
        }

        add_systems.push(world.register_system(add_to_cam::<T>));
        rem_systems.push(world.register_system(remove_from_cam::<T>));

        let mut cam_manager = world.resource_mut::<CameraManager>();

        if cloned.is_some() {
            cam_manager.cam_styles.insert(TypeId::of::<T>(), cloned.unwrap().clone());
        }
        cam_manager.update.insert(TypeId::of::<T>(), update_systems);
        cam_manager.on_enter.insert(TypeId::of::<T>(), add_systems);
        cam_manager.on_exit.insert(TypeId::of::<T>(), rem_systems);

        // register
        cam_manager.add_layer::<T>();
    }
}

impl<'w> CameraBuilder<'w> {
    pub fn config_layer<T: Default + Component>(self, mut state: CamLayerConfig<T>) -> Self {
        state.apply_config(self.app);

        self
    }
    pub fn clear_color(self, color: impl Into<Color>) -> Self {
        self.app.insert_resource(ClearColor(color.into()));

        self
    }
    pub fn app(self) -> &'w mut App {
        self.app
    }
}

impl CameraManagerBuildExt for App {
    fn camera_manager(&mut self) -> CameraBuilder {
        CameraBuilder { app: self }
    }
}