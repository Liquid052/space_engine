use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

pub fn print_max_texture_size(render_device: Res<RenderDevice>) {
    let limits = render_device.limits();
    let max_texture_size = limits.max_texture_dimension_2d;
    info!("Max texture size supported: {}x{}", max_texture_size, max_texture_size);
}