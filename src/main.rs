use mesh::Mesh;
use voxel::Axis;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::mesh::FaceMeshes;
use crate::mesh::Uniforms;
use crate::mesh::Vertex;
use crate::voxel::Chunk;

mod camera;
mod mesh;
mod texture;
mod voxel;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swapchain_desc: wgpu::SwapChainDescriptor,
    swapchain: wgpu::SwapChain,
    depth_texture: texture::Texture,
    pipeline: wgpu::RenderPipeline,
    mesh_buffers: Vec<AxisMeshBuffer>,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_group: wgpu::BindGroup,
    camera: camera::Camera,
    controller: camera::CameraController,
}

struct AxisMeshBuffer {
    axis: Axis,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    vertex_count: u32,
    index_count: u32,
}

impl AxisMeshBuffer {
    fn new(device: &wgpu::Device, axis: Axis, mesh: Mesh) -> Self {
        let vertex_count = mesh.vertices.len() as u32;
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
            axis,
            vertices,
            indices,
            vertex_count,
            index_count,
        }
    }
}

impl State {
    async fn new(window: &Window) -> Self {
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

        let mut chunk = Chunk::new(0, 0, 0);
        chunk.randomize();
        let meshes = FaceMeshes::new(&chunk);
        let mesh_buffers: Vec<AxisMeshBuffer> = vec![
            AxisMeshBuffer::new(&device, Axis::Zneg, meshes.z_neg),
            AxisMeshBuffer::new(&device, Axis::Zpos, meshes.z_pos),
            AxisMeshBuffer::new(&device, Axis::Yneg, meshes.y_neg),
            AxisMeshBuffer::new(&device, Axis::Ypos, meshes.y_pos),
            AxisMeshBuffer::new(&device, Axis::Xneg, meshes.x_neg),
            AxisMeshBuffer::new(&device, Axis::Xpos, meshes.x_pos),
        ];
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        };
        let camera = camera::Camera::new(width, height);
        let controller = camera::CameraController::new(0.5);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);
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
                front_face: wgpu::FrontFace::Ccw, // TODO for some reason my winding order is wrong?!
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
        Self {
            surface,
            device,
            queue,
            swapchain_desc,
            swapchain,
            depth_texture,
            pipeline,
            mesh_buffers,
            uniforms,
            uniform_buffer,
            uniform_group,
            camera,
            controller,
        }
    }

    fn update(&mut self) {
        self.controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn input(&mut self, event: &WindowEvent) {
        self.controller.process_events(event);
    }

    fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.swapchain_desc.width = new_size.width;
        self.swapchain_desc.height = new_size.height;
        self.swapchain = self
            .device
            .create_swap_chain(&self.surface, &self.swapchain_desc);
        self.depth_texture =
            texture::Texture::new_depth_texture(&self.device, &self.swapchain_desc);
        self.camera
            .update_aspect(new_size.width as f32, new_size.height as f32);
        self.controller.update_camera(&mut self.camera);
    }

    fn redraw(&self) {
        let frame = match self.swapchain.get_current_frame() {
            Ok(frame) => frame.output,
            Err(wgpu::SwapChainError::OutOfMemory) => panic!("Out of memory!"),
            Err(_) => return, // Handled on the next frame
        };
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
        self.queue.submit(Some(encoder.finish()));
    }
}

fn main() {
    use winit::event::Event;
    use winit::event_loop::{ControlFlow, EventLoop};

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("voxel-planet")
        .build(&event_loop)
        .unwrap();
    let mut state = futures::executor::block_on(State::new(&window));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                state.redraw();
            }
            Event::WindowEvent { window_id, event } if window.id() == window_id => match event {
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(new_inner_size)
                }
                WindowEvent::Resized(ref new_size) => state.resize(new_size),
                WindowEvent::KeyboardInput { .. } => state.input(&event),

                // Unused (I like exhaustive cases!)
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::MouseInput { .. } => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::Moved(_) => {}
                WindowEvent::Destroyed => {}
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::ReceivedCharacter(_) => {}
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::TouchpadPressure { .. } => {}
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::ThemeChanged(_) => {}
            },
            Event::RedrawRequested(_) => {}
            Event::WindowEvent { .. } => {}
            Event::NewEvents(_) => {}
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    });
}
