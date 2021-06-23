use futures::executor::{LocalPool, LocalSpawner};
use wgpu::{
    util::StagingBelt, Device, Queue, Surface, SwapChain, SwapChainDescriptor, TextureView,
};
use wgpu_glyph::GlyphBrush;

pub struct Brush<'a> {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub swapchain_desc: SwapChainDescriptor,
    pub swapchain: SwapChain,
    pub glyph_brush: GlyphBrush<()>,
    pub staging_belt: StagingBelt,
    pub local_pool: LocalPool,
    pub local_spawner: LocalSpawner,
    pub color_target: &'a TextureView,
}

impl<'a> Brush<'a> {
    pub fn new() -> Self {
        todo!()
    }
}
