use wgpu::util::DeviceExt;

pub struct WidgetPainter {
    pub slider_pipeline: wgpu::RenderPipeline,

    slider: Slider,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl WidgetPainter {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        // [[location(0)]] position: vec2<f32>
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        };
        // [[location(1)]] size: vec2<f32>
        // [[location(2)]] progress: f32
        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as u64,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32],
        };
        let slider_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
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
            slider,
            vertex_buffer,
            vertex_count,
            instance_buffer,
            instance_count,
        }
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
            y: 90.0,
            width: 500.0,
            height: 50.0,
            progress: 0.2,
        }
    }
}

impl Slider {
    fn as_quad(&self) -> [Vertex; 6] {
        const W: f32 = 600.0;
        const H: f32 = 800.0;
        let x = self.x / W - W / 2.0;
        let y = self.y / H - H / 2.0;
        let width = self.width / W;
        let height = self.height / H;
        let bottom_left = Vertex { position: [x, y] };
        let bottom_right = Vertex {
            position: [x + width, y],
        };
        let top_right = Vertex {
            position: [x + width, y + height],
        };
        let top_left = Vertex {
            position: [x, y + height],
        };
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
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    size: [f32; 2],
    progress: f32,
}
