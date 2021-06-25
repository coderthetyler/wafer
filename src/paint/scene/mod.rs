use wgpu::{
    util::DeviceExt, Color, CommandBuffer, CommandEncoderDescriptor, Device, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    SwapChainDescriptor,
};

use crate::camera::Camera;

use self::voxels::VoxelPainter;

use super::{texture::Texture, PaintContext};

mod colliders;
mod voxels;

/// Represents world uniforms common to all world painters.
/// For example, this includes the view projection matrix.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct WorldUniforms {
    view_proj: [[f32; 4]; 4],
}

impl WorldUniforms {
    fn new() -> Self {
        Self {
            view_proj: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

/// Responsible for rendering in world space.
pub struct ScenePainter {
    depth_texture: Texture,

    voxel_painter: VoxelPainter,

    uniforms: WorldUniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_group: wgpu::BindGroup,
}

impl ScenePainter {
    pub fn new(device: &Device, swapchain_desc: &SwapChainDescriptor) -> Self {
        let uniforms = WorldUniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let uniform_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let uniform_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &uniform_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let depth_texture = Texture::new_depth_texture(device, &swapchain_desc);
        let voxel_painter = VoxelPainter::new(device, swapchain_desc, &uniform_group_layout);
        Self {
            depth_texture,
            voxel_painter,
            uniforms,
            uniform_buffer,
            uniform_group,
        }
    }

    pub fn update_swapchain(&mut self, device: &Device, swapchain_desc: &SwapChainDescriptor) {
        self.depth_texture = Texture::new_depth_texture(device, swapchain_desc);
    }

    pub fn paint(&mut self, ctx: &mut PaintContext, camera: &Camera) -> CommandBuffer {
        self.uniforms.view_proj = camera
            .build_view_projection_matrix((ctx.width, ctx.height).into())
            .into();
        ctx.surface.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        let mut encoder = ctx
            .surface
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: ctx.color_target,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.4,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            // Voxel painter
            {
                let pntr = &mut self.voxel_painter;
                render_pass.set_pipeline(&pntr.pipeline);
                render_pass.set_bind_group(0, &self.uniform_group, &[]);
                for buf in &pntr.mesh_buffers {
                    render_pass.set_vertex_buffer(0, buf.vertices.slice(..));
                    render_pass.set_index_buffer(buf.indices.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..buf.index_count, 0, 0..1);
                }
            }
        }
        encoder.finish()
    }
}