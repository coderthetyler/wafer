use wgpu::util::DeviceExt;

pub struct WidgetPainter {
    pub slider_pipeline: wgpu::RenderPipeline,

    uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_group: wgpu::BindGroup,

    slider: Slider,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl WidgetPainter {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        // uniform buffer & pals
        let uniforms = Uniforms::default();
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

        // [[location(0)]] position: vec2<f32>
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        };
        // [[location(1)]] size: vec2<f32>
        // [[location(2)]] progress: f32
        // [[location(3)]] left: f32;
        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32, 3 => Float32x2],
        };
        let slider_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&uniform_group_layout],
                push_constant_ranges: &[],
            });
        let slider_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/slider.wgsl").into()),
            flags: wgpu::ShaderFlags::all(),
        });
        let slider_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&slider_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &slider_shader,
                entry_point: "vs_main",
                buffers: &[vertex_layout, instance_layout],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &slider_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: None,
                    write_mask: wgpu::ColorWrite::all(),
                }],
            }),
        });

        // dummy data for slider testing
        let slider = Slider::default();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&slider.as_quad()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let vertex_count = 6;
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[slider.as_instance()]),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let instance_count = 1;
        Self {
            slider_pipeline,
            uniforms,
            uniform_buffer,
            uniform_group,
            slider,
            vertex_buffer,
            vertex_count,
            instance_buffer,
            instance_count,
        }
    }

    pub fn update_uniforms(&mut self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.uniforms.ortho_proj = cgmath::ortho(0.0, width, height, 0.0, -1.0, 1.0).into();
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}

pub struct Slider {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    progress: f32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 100.0,
            width: 400.0,
            height: 50.0,
            progress: 0.2,
        }
    }
}

impl Slider {
    fn as_quad(&self) -> [Vertex; 6] {
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;
        let bottom_left = Vertex {
            position: [x, y + height],
        };
        let bottom_right = Vertex {
            position: [x + width, y + height],
        };
        let top_right = Vertex {
            position: [x + width, y],
        };
        let top_left = Vertex { position: [x, y] };
        [
            bottom_left,
            bottom_right,
            top_right,
            top_right,
            top_left,
            bottom_left,
        ]
    }

    fn as_instance(&self) -> Instance {
        Instance {
            size: [self.width, self.height],
            progress: self.progress,
            top_left: [self.x, self.y],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    size: [f32; 2],
    progress: f32,
    top_left: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    ortho_proj: [[f32; 4]; 4],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            ortho_proj: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
