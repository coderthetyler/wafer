use futures::{
    executor::{LocalPool, LocalSpawner},
    task::SpawnExt,
};
use wgpu::{
    util::StagingBelt, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, LoadOp,
    Operations, RenderPassColorAttachment, RenderPassDescriptor, SwapChainDescriptor,
    TextureFormat, TextureView,
};
use wgpu_glyph::{
    ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, GlyphPositioner, Layout, Section,
    SectionGeometry, SectionText, Text,
};

use crate::console::Console;

pub struct ConsoleDrawSubsystem {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    local_pool: LocalPool,
    local_spawner: LocalSpawner,
}

impl ConsoleDrawSubsystem {
    pub fn new(device: &wgpu::Device, swapchain_desc: &SwapChainDescriptor) -> Self {
        // Stuff for text rendering
        let staging_belt = StagingBelt::new(1024);
        let local_pool = LocalPool::new();
        let local_spawner = local_pool.spawner();
        let font = FontArc::try_from_slice(include_bytes!("Tuffy.ttf")).unwrap();
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&device, TextureFormat::Bgra8UnormSrgb);
        Self {
            glyph_brush,
            staging_belt,
            local_pool,
            local_spawner,
        }
    }

    pub fn recycle(&mut self) {
        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");
        self.local_pool.run_until_stalled();
    }

    pub fn draw(
        &mut self,
        device: &Device,
        color_target: &TextureView,
        bounds: (u32, u32),
        console: &Console,
    ) -> CommandBuffer {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        self.draw_cursor(color_target, bounds, &mut encoder, console);
        self.draw_text(device, color_target, bounds, &mut encoder, console);
        encoder.finish()
    }

    fn draw_cursor(
        &mut self,
        color_target: &TextureView,
        (width, height): (u32, u32),
        encoder: &mut CommandEncoder,
        console: &Console,
    ) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[RenderPassColorAttachment {
                view: color_target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        let prompt_text = SectionText {
            text: console.get_text().as_str(),
            scale: 40.0.into(),
            font_id: wgpu_glyph::FontId(0),
        };
        let prompt_glyphs = Layout::default().calculate_glyphs(
            self.glyph_brush.fonts(),
            &SectionGeometry::default(),
            &[prompt_text],
        );
        // TODO calculate cursor screen position from prompt glyphs
        // TODO upload uniform containing info on new screen position
    }

    fn draw_text(
        &mut self,
        device: &Device,
        color_target: &TextureView,
        (width, height): (u32, u32),
        encoder: &mut CommandEncoder,
        console: &Console,
    ) {
        let prompt_text = Text::new(console.get_text().as_str())
            .with_color([0.0, 0.0, 0.0, 1.0])
            .with_scale(40.0);
        let prompt_section = Section::default()
            .with_screen_position((30.0, 30.0))
            .with_bounds((width as f32, height as f32))
            .add_text(prompt_text);
        self.glyph_brush.queue(prompt_section);
        self.glyph_brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                encoder,
                color_target,
                width,
                height,
            )
            .expect("Draw queued");
        self.staging_belt.finish();
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    cursor_position: [f32; 2],
}

impl Uniforms {
    fn new() -> Self {
        Self {
            cursor_position: [0.0, 0.0],
        }
    }
}