use crate::prelude::CameraManager;
use crate::systems::{add_to_cam, remove_from_cam, update_style};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::any::{type_name, TypeId};


/// Trait for extending the App with camera manager functionalities.
pub trait CameraManagerBuildExt {
    /// Initializes the camera manager and returns a CameraBuilder instance.
    fn camera_manager(&mut self) -> CameraBuilder;
}

/// Builder struct for configuring the camera state.
pub struct CameraBuilder<'w> {
    #[doc(hidden)]
    app: &'w mut App,
}



/// Configuration for a specific camera layer.
#[derive(Default, Clone)]
pub struct CamLayerConfig<T> {
    _marker: T, // Marker type for the camera layer

    style: Option<CameraStyle>, // Optional style configuration for the camera
}

/// Style settings for the camera.
#[derive(Default, Clone)]
pub struct CameraStyle {
    /// Zoom limits for the camera.
    pub clamp_zoom: Option<(f32, f32)>,
    /// Tonemapping method to be applied when the camera is entered.
    pub tone_mapping: Option<Tonemapping>,
    /// Render layer for the camera.
    pub render_layer: Option<RenderLayers>,
    /// Bloom settings for the camera.
    pub bloom: Option<BloomSettings>,
    /// Flag to enable HDR rendering.
    pub hdr: bool,
    /// Depth of the camera layer.
    pub depth: Option<f32>,
}


impl<T: Default + Component> CamLayerConfig<T> {
    /// Creates a new CamLayerConfig with the specified marker type.
    pub fn new(marker: T) -> Self {
        Self {
            _marker: marker,
            style: None,
        }
    }

    /// Retrieves or creates the CameraStyle for this configuration.
    fn get_or_create(&mut self) -> &mut CameraStyle {
        if self.style.is_none() {
            self.style = Some(CameraStyle::default());
        }

        self.style.as_mut().unwrap()
    }

    /// Sets the tonemapping method for the camera style.
    pub fn tone_mapping(mut self, tonemapping: Tonemapping) -> Self {
        self.get_or_create().tone_mapping = Some(tonemapping);
        self
    }

    /// Sets the bloom settings for the camera style.
    pub fn bloom(mut self, bloom: BloomSettings) -> Self {
        self.get_or_create().bloom = Some(bloom);
        self
    }

    /// Sets the zoom limits for the camera style.
    pub fn clamp_zoom(mut self, min: f32, max: f32) -> Self {
        self.get_or_create().clamp_zoom = Some((min, max));
        self
    }

    /// Sets the render layer for the camera style.
    pub fn render_layer(mut self, layer: RenderLayers) -> Self {
        self.get_or_create().render_layer = Some(layer);
        self
    }

    /// Sets the depth for the camera style.
    pub fn depth(mut self, depth: f32) -> Self {
        self.get_or_create().depth = Some(depth);
        self
    }

    /// Enables HDR rendering for the camera style.
    pub fn enable_hdr(mut self) -> Self {
        self.get_or_create().hdr = true;
        self
    }

    /// Applies the camera layer configuration to the app.
    ///
    /// # Parameters
    /// - `config`: The app configuration to which the layer will be applied.
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

        // Register the camera layer
        cam_manager.add_layer::<T>();
    }
}

impl<'w> CameraBuilder<'w> {
    /// Configures a camera layer with the specified CamLayerConfig.
    ///
    /// # Parameters
    /// - `state`: The configuration for the camera layer.
    pub fn config_layer<T: Default + Component>(self, mut state: CamLayerConfig<T>) -> Self {
        state.apply_config(self.app);
        self
    }

    /// Sets the clear color for the camera.
    ///
    /// # Parameters
    /// - `color`: The color to set as the clear color.
    pub fn clear_color(self, color: impl Into<Color>) -> Self {
        self.app.insert_resource(ClearColor(color.into()));
        self
    }

    /// Returns a mutable reference to the app.
    pub fn app(self) -> &'w mut App {
        self.app
    }
}

impl CameraManagerBuildExt for App {
    /// Extends the App with camera manager functionalities.
    fn camera_manager(&mut self) -> CameraBuilder {
        CameraBuilder { app: self }
    }
}