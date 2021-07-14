mod widgets;

use futures::{
    executor::{LocalPool, LocalSpawner},
    task::SpawnExt,
};
use wgpu::{
    util::StagingBelt, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device,
    TextureFormat, TextureView,
};
use wgpu_glyph::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Section, Text};

use crate::{app::AppConfig, frame::Frame, session::ConsoleSession};

use self::widgets::WidgetPainter;

/// Responsible for rendering an overlay.
/// This includes rendering any UI or debugging info.
pub struct OverlayPainter {
    widget_painter: WidgetPainter,
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    local_pool: LocalPool,
    local_spawner: LocalSpawner,
}

impl OverlayPainter {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        // Stuff for text rendering
        let staging_belt = StagingBelt::new(1024);
        let local_pool = LocalPool::new();
        let local_spawner = local_pool.spawner();
        let font = FontArc::try_from_slice(include_bytes!("fonts/Tuffy.ttf")).unwrap();
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&device, TextureFormat::Bgra8UnormSrgb);
        Self {
            widget_painter: WidgetPainter::new(device, sc_desc),
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
        config: &AppConfig,
        frame: &Frame,
        device: &Device,
        color_target: &TextureView,
        bounds: (u32, u32),
        session: &ConsoleSession,
        triangle_count: usize,
    ) -> CommandBuffer {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

        if session.is_showing() {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: color_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            {
                let pntr = &self.widget_painter;
                render_pass.set_pipeline(&pntr.slider_pipeline);
                render_pass.set_vertex_buffer(0, pntr.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, pntr.instance_buffer.slice(..));
                render_pass.draw(0..pntr.vertex_count, 0..pntr.instance_count);
            }
        }
        if session.is_showing() {
            let x = 10.0;
            let mut y = bounds.1 as f32 - 50.0;
            self.draw_text(
                device,
                color_target,
                bounds,
                &mut encoder,
                session.console.get_text().as_str(),
                (x, y),
            );
            for entry in session.console.history_newest_first() {
                y -= 42.0;
                if y <= 80.0 {
                    break;
                }
                self.draw_text(
                    device,
                    color_target,
                    bounds,
                    &mut encoder,
                    entry.as_str(),
                    (x, y),
                );
            }
        }
        if !config.hide_debug_overlay {
            self.draw_text(
                device,
                color_target,
                bounds,
                &mut encoder,
                format!(
                    "fps: {}\nfaces: {}",
                    (frame.framerate * 10.0).round() / 10.0,
                    triangle_count
                )
                .as_str(),
                (10.0, 10.0),
            );
        }

        self.staging_belt.finish();
        encoder.finish()
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
