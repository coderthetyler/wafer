use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, BufferUsage, Device, RenderPipeline, ShaderFlags, ShaderModuleDescriptor,
    ShaderSource, SwapChainDescriptor,
};

use crate::{
    geometry::Vec3f,
    paint::texture::Texture,
    planets::{
        chunk::{Chunk, CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z},
        voxel::Voxel,
    },
};

/// Responsible for drawing terrain voxels.
pub struct VoxelPainter {
    pub pipeline: RenderPipeline,
    pub mesh_buffers: Vec<IndexedVertexBuffer>,
}

impl VoxelPainter {
    pub fn new(
        device: &Device,
        swapchain_desc: &SwapChainDescriptor,
        world_uniforms_layout: &BindGroupLayout,
    ) -> Self {
        let mut chunk = Chunk::new([0, 0, 0].into());
        chunk.randomize();
        let chunk_mesh = chunk.build_mesh();
        let mesh_buffers = vec![IndexedVertexBuffer::new(&device, chunk_mesh)];
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VoxelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        };
        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            flags: ShaderFlags::all(),
            source: ShaderSource::Wgsl(include_str!("shaders/voxels.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[world_uniforms_layout],
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
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
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
        Self {
            pipeline,
            mesh_buffers,
        }
    }
}

pub struct IndexedVertexBuffer {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub index_count: u32,
}

impl IndexedVertexBuffer {
    fn new(device: &Device, mesh: VoxelMesh) -> Self {
        let index_count = mesh.indices.len() as u32;
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&mesh.vertices),
            usage: BufferUsage::VERTEX,
        });
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: BufferUsage::INDEX,
        });
        Self {
            vertices,
            indices,
            index_count,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct VoxelVertex {
    position: Vec3f,
    color: Vec3f,
}

struct VoxelMesh {
    vertices: Vec<VoxelVertex>,
    indices: Vec<u32>,
}

impl Chunk {
    fn build_mesh(&self) -> VoxelMesh {
        let faces: Vec<_> = (0..6)
            .into_iter()
            .map(|axis_index| {
                let axis = &AXES[axis_index];
                let unit_face = &VOXEL_FACES[axis_index];
                self.iter()
                    .filter(move |(coord, voxel)| {
                        !voxel.is_air()
                            && matches!(self.get_voxel(coord + axis), Some(Voxel::Air) | None)
                    })
                    .map(move |(coord, voxel)| {
                        let color: [f32; 3] = match voxel {
                            Voxel::Air => unreachable!(),
                            Voxel::Grass => [0.33, 0.80, 0.46],
                            Voxel::Dirt => [0.35, 0.29, 0.21],
                        };
                        let weight: f32 = rand::random();
                        let color: Vec3f =
                            [color[0] * weight, color[1] * weight, color[2] * weight].into();
                        let base: Vec3f = [
                            (self.coord.x * CHUNK_SIZE_X + coord.x) as f32,
                            (self.coord.y * CHUNK_SIZE_Y + coord.y) as f32,
                            (self.coord.z * CHUNK_SIZE_Z + coord.z) as f32,
                        ]
                        .into();
                        [
                            VoxelVertex {
                                position: base + unit_face[0],
                                color,
                            },
                            VoxelVertex {
                                position: base + unit_face[1],
                                color,
                            },
                            VoxelVertex {
                                position: base + unit_face[2],
                                color,
                            },
                            VoxelVertex {
                                position: base + unit_face[3],
                                color,
                            },
                        ]
                    })
            })
            .flatten()
            .collect();

        let mut vertices: Vec<VoxelVertex> = Vec::with_capacity(faces.len() * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(faces.len() * 6);
        let mut index: u32 = 0;

        #[allow(clippy::identity_op)]
        for face in faces {
            vertices.push(face[0]);
            vertices.push(face[1]);
            vertices.push(face[2]);
            vertices.push(face[3]);
            indices.push(index + 0);
            indices.push(index + 2);
            indices.push(index + 1);
            indices.push(index + 0);
            indices.push(index + 3);
            indices.push(index + 2);
            index += 4;
        }
        VoxelMesh { vertices, indices }
    }
}

const AXES: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

const VOXEL_FACES: [[[f32; 3]; 4]; 6] = [
    [
        // +X
        [1.0, 0.0, 0.0], // bottom-left
        [1.0, 0.0, 1.0], // bottom-right
        [1.0, 1.0, 1.0], // top-right
        [1.0, 1.0, 0.0], // top-left
    ],
    [
        // -X
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
    ],
    [
        // +Y
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ],
    [
        // -Y
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ],
    [
        // +Z
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
    ],
    [
        // -Z
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ],
];
