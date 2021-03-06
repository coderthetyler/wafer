mod volumes;

use cgmath::{Deg, Vector3};
use wgpu::{
    util::DeviceExt, Color, CommandBuffer, CommandEncoderDescriptor, Device, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    SwapChainDescriptor,
};

use crate::{
    app::AppConfig,
    camera::Camera,
    entity::{Ecs, Entity},
    types::{AspectRatio, Position, Rotation},
};

use self::volumes::VolumePainter;

use super::{texture::Texture, PaintContext};

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

    volume_painter: VolumePainter,

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
        let volume_painter = VolumePainter::new(device, swapchain_desc, &uniform_group_layout);
        Self {
            depth_texture,
            volume_painter,
            uniforms,
            uniform_buffer,
            uniform_group,
        }
    }

    pub fn update_swapchain(&mut self, device: &Device, swapchain_desc: &SwapChainDescriptor) {
        self.depth_texture = Texture::new_depth_texture(device, swapchain_desc);
    }

    pub fn build_view_projection_matrix(
        &self,
        camera: &Camera,
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        aspect_ratio: AspectRatio,
    ) -> cgmath::Matrix4<f32> {
        let pitch = rotation.x;
        let yaw = rotation.y;
        let view = cgmath::Matrix4::from_angle_x(Deg(pitch))
            * cgmath::Matrix4::from_angle_y(Deg(yaw))
            * cgmath::Matrix4::from_translation(position);
        let proj = cgmath::perspective(
            Deg(camera.fovy),
            aspect_ratio.into(),
            camera.znear,
            camera.zfar,
        );
        proj * view
    }

    pub fn paint(
        &mut self,
        config: &AppConfig,
        ctx: &mut PaintContext,
        viewer: Entity,
        ecs: &Ecs,
    ) -> CommandBuffer {
        let pos = ecs.comps.position.get(viewer).unwrap_or(&Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let pos = [pos.x, pos.y, pos.z].into();
        let rot = ecs.comps.rotation.get(viewer).unwrap_or(&Rotation {
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        });
        let rot = [rot.pitch, rot.yaw, rot.roll].into();
        let camera = ecs.comps.camera.get(viewer).unwrap_or(&Camera {
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
        });
        let aspect_ratio = (ctx.width, ctx.height).into();
        self.uniforms.view_proj = self
            .build_view_projection_matrix(camera, pos, rot, aspect_ratio)
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
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
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

            if !config.hide_volumes {
                let pntr = &mut self.volume_painter;
                pntr.update(&ctx.surface.device, ecs);
                render_pass.set_pipeline(&pntr.pipeline);
                render_pass.set_bind_group(0, &self.uniform_group, &[]);
                render_pass.set_vertex_buffer(0, pntr.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, pntr.instance_buffer.slice(..));
                render_pass
                    .set_index_buffer(pntr.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..pntr.index_count, 0, 0..pntr.instance_count);
            }
        }
        encoder.finish()
    }
}
