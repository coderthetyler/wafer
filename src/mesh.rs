use crate::voxel::{Chunk, Voxel, CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z};

type Vec3f = [f32; 3];
fn add(a: Vec3f, b: Vec3f) -> Vec3f {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

type Vec3i = [i32; 3];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3f,
    pub color: Vec3f,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

const AXES: [Vec3i; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

const VOXEL_FACES: [[Vec3f; 4]; 6] = [
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

impl Chunk {
    pub fn build_mesh(&self) -> Mesh {
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
                            [color[0] * weight, color[1] * weight, color[2] * weight];
                        let base: Vec3f = [
                            (self.coord.x * CHUNK_SIZE_X + coord.x) as f32,
                            (self.coord.y * CHUNK_SIZE_Y + coord.y) as f32,
                            (self.coord.z * CHUNK_SIZE_Z + coord.z) as f32,
                        ];
                        [
                            Vertex {
                                position: add(base, unit_face[0]),
                                color,
                            },
                            Vertex {
                                position: add(base, unit_face[1]),
                                color,
                            },
                            Vertex {
                                position: add(base, unit_face[2]),
                                color,
                            },
                            Vertex {
                                position: add(base, unit_face[3]),
                                color,
                            },
                        ]
                    })
            })
            .flatten()
            .collect();

        let mut vertices: Vec<Vertex> = Vec::with_capacity(faces.len() * 4);
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
        Mesh { vertices, indices }
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
