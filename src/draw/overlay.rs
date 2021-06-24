use futures::{
    executor::{LocalPool, LocalSpawner},
    task::SpawnExt,
};
use wgpu::{
    util::StagingBelt, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, LoadOp,
    Operations, RenderPassColorAttachment, RenderPassDescriptor, TextureFormat, TextureView,
};
use wgpu_glyph::{
    ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, GlyphPositioner, Layout, Section,
    SectionGeometry, SectionText, Text,
};

use crate::{console::Console, time::Frame};

pub struct OverlaySubsystem {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    local_pool: LocalPool,
    local_spawner: LocalSpawner,
    pub show_debug_overlay: bool,
}

impl OverlaySubsystem {
    pub fn new(device: &wgpu::Device) -> Self {
        // Stuff for text rendering
        let staging_belt = StagingBelt::new(1024);
        let local_pool = LocalPool::new();
        let local_spawner = local_pool.spawner();
        let font = FontArc::try_from_slice(include_bytes!("fonts/Tuffy.ttf")).unwrap();
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&device, TextureFormat::Bgra8UnormSrgb);
        Self {
            glyph_brush,
            staging_belt,
            local_pool,
            local_spawner,
            show_debug_overlay: true,
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
        frame: &Frame,
        device: &Device,
        color_target: &TextureView,
        bounds: (u32, u32),
        console: &Console,
        triangle_count: usize,
    ) -> CommandBuffer {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        if console.is_showing() {
            self.draw_cursor(color_target, bounds, &mut encoder, console);
            self.draw_text(
                device,
                color_target,
                bounds,
                &mut encoder,
                console.get_text().as_str(),
                (10.0, bounds.1 as f32 - 50.0),
            );
        }
        if self.show_debug_overlay {
            self.draw_text(
                device,
                color_target,
                bounds,
                &mut encoder,
                format!(
                    "fps: {}\nfaces: {}",
                    frame.framerate.round() as u32,
                    triangle_count
                )
                .as_str(),
                (10.0, 10.0),
            );
        }

        self.staging_belt.finish();
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
        text: &str,
        position: (f32, f32),
    ) {
        let prompt_text = Text::new(text)
            .with_color([0.0, 0.0, 0.0, 1.0])
            .with_scale(35.0);
        let prompt_section = Section::default()
            .with_screen_position(position)
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
