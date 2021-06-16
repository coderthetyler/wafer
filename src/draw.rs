use ascii::AsAsciiStr;
use futures::executor::{LocalPool, LocalSpawner};
use wgpu::util::{DeviceExt, StagingBelt};
use wgpu_glyph::GlyphBrushBuilder;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{camera::Camera, console::Console, geometry::Vec3f, planets::chunk::Chunk, texture};

pub struct DrawSystem {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swapchain_desc: wgpu::SwapChainDescriptor,
    swapchain: wgpu::SwapChain,
    depth_texture: texture::Texture,
    pipeline: wgpu::RenderPipeline,
    mesh_buffers: Vec<IndexedVertexBuffer>,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_group: wgpu::BindGroup,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
    staging_belt: StagingBelt,
    local_pool: LocalPool,
    local_spawner: LocalSpawner,
}

impl DrawSystem {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let PhysicalSize { width, height } = window.inner_size();
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();
        let swapchain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swapchain = device.create_swap_chain(&surface, &swapchain_desc);
        let depth_texture = texture::Texture::new_depth_texture(&device, &swapchain_desc);
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let mut chunk = Chunk::new([0, 0, 0].into());
        chunk.randomize();
        let chunk_mesh = chunk.build_mesh();
        let mesh_buffers = vec![IndexedVertexBuffer::new(&device, chunk_mesh)];
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        };
        let uniforms = Uniforms::new();
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
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_layout],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: swapchain_format,
                    blend: None,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();
        let font =
            wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!("Tuffy.ttf")).unwrap();
        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

        Self {
            surface,
            device,
            queue,
            swapchain_desc,
            swapchain,
            depth_texture,
            pipeline,
            uniforms,
            uniform_buffer,
            uniform_group,
            mesh_buffers,
            glyph_brush,
            staging_belt,
            local_pool,
            local_spawner,
        }
    }

    pub fn resize_surface(&mut self, new_size: &PhysicalSize<u32>) {
        self.swapchain_desc.width = new_size.width;
        self.swapchain_desc.height = new_size.height;
        self.swapchain = self
            .device
            .create_swap_chain(&self.surface, &self.swapchain_desc);
        self.depth_texture =
            texture::Texture::new_depth_texture(&self.device, &self.swapchain_desc);
    }

    fn draw_console(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        console: &Console,
    ) {
        let width = self.swapchain_desc.width;
        let height = self.swapchain_desc.height;
        self.glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (30.0, 30.0),
            bounds: (width as f32, height as f32),
            text: vec![
                wgpu_glyph::Text::new(console.get_text().slice_ascii(..).unwrap().into())
                    .with_color([0.0, 0.0, 0.0, 1.0])
                    .with_scale(40.0),
            ],
            ..wgpu_glyph::Section::default()
        });
        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                encoder,
                view,
                width,
                height,
            )
            .expect("Draw queued");

        self.staging_belt.finish();
    }

    pub fn redraw(&mut self, camera: &Camera, console: &Console) {
        let frame = match self.swapchain.get_current_frame() {
            Ok(frame) => frame.output,
            Err(wgpu::SwapChainError::OutOfMemory) => panic!("Out of memory!"),
            Err(_) => return, // Handled on the next frame
        };

        // Build view projection matrix from camera
        let width = self.swapchain_desc.width as f32;
        let height = self.swapchain_desc.height as f32;
        self.uniforms.view_proj = camera
            .build_view_projection_matrix((width, height).into())
            .into();
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        // Build command buffer for the frame
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.4,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.uniform_group, &[]);
            for buf in &self.mesh_buffers {
                render_pass.set_vertex_buffer(0, buf.vertices.slice(..));
                render_pass.set_index_buffer(buf.indices.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..buf.index_count, 0, 0..1);
            }
        }
        if console.is_showing() {
            self.draw_console(&mut encoder, &frame.view, console);
        }
        self.queue.submit(Some(encoder.finish()));

        use futures::task::SpawnExt;
        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");
        self.local_pool.run_until_stalled();
    }
}

pub struct IndexedVertexBuffer {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub index_count: u32,
}

impl IndexedVertexBuffer {
    pub fn new(device: &wgpu::Device, mesh: Mesh) -> Self {
        let index_count = mesh.indices.len() as u32;
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        Self {
            vertices,
            indices,
            index_count,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: Vec3f,
    pub color: Vec3f,
}

pub struct Mesh {
    pub vertices: Vec<ColorVertex>,
    pub indices: Vec<u32>,
}
