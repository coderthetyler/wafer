
use cgmath::{Matrix4, SquareMatrix};
use wgpu::{util::DeviceExt, BindGroupLayout, Device};

use crate::{entity::{Entity, EntityComponents, EntityPool}, types::Volume, paint::texture::Texture};

pub struct VolumePainter {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl VolumePainter {
    pub fn new(
        device: &Device,
        swapchain_desc: &wgpu::SwapChainDescriptor,
        world_uniforms_layout: &BindGroupLayout,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[world_uniforms_layout],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/box.wgsl").into()),
            flags: wgpu::ShaderFlags::all(),
        });
        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![
                1 => Float32x4, 
                2 => Float32x4, 
                3 => Float32x4, 
                4 => Float32x4],
        };
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexData>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_layout, instance_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: swapchain_desc.format,
                    blend: None,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
        });
        let box_vertices: [[f32; 3]; 8] = [
            [0.0, 0.0, 0.0], // 0
            [0.0, 1.0, 0.0], // 1
            [0.0, 1.0, 1.0], // 2
            [0.0, 0.0, 1.0], // 3
            [1.0, 0.0, 0.0], // 4
            [1.0, 1.0, 0.0], // 5
            [1.0, 1.0, 1.0], // 6
            [1.0, 0.0, 1.0], // 7
        ];
        let box_indices: [u16; 24] = [
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 3, 7, 2, 6
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&box_vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&box_indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let index_count = box_indices.len() as u32;
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[],
            usage: wgpu::BufferUsage::VERTEX,
        });
        let instance_count = 0;
        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
            instance_buffer,
            instance_count,
        }
    }

    /// Update the painter with updated collider info
    pub fn update(&mut self, device: &Device, entities: &EntityPool, components: &EntityComponents) {
        /// Interpret an entity as an instance to draw
        fn entity_to_instance(entity: Entity, components: &EntityComponents) -> Option<InstanceData> {
            if let (Some(pos), Some(Volume::Box { x, y, z })) =
                (components.position.get(entity), components.colliders.get(entity))
            {
                let mut model: Matrix4<f32> = cgmath::Matrix4::identity();

                // Scale
                let scale = cgmath::Matrix4::from_nonuniform_scale(*x, *y, *z);
                model = scale * model;

                // Position
                let translation =
                    cgmath::Matrix4::from_translation([pos.x, pos.y, pos.z].into());
                model = translation * model;
                
                Some(InstanceData {
                    model: model.into(),
                })
            } else {
                None
            }
        }

        // Construct instance buffer from box instances
        let box_instances: Vec<InstanceData> = entities
            .iter()
            .filter_map(|entity| entity_to_instance(entity, components))
            .collect();
        self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&box_instances),
            usage: wgpu::BufferUsage::VERTEX,
        });
        self.instance_count = box_instances.len() as u32;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexData {
    position: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    model: [[f32; 4]; 4],
}

impl InstanceData {
    fn new() -> Self {
        Self {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
