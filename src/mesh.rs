use crate::{
    camera::Camera,
    voxel::{Axis, Chunk, Coordinate, Voxel, CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z},
};

type Vec3 = [f32; 3];
fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

struct Face {
    bottom_left: Vertex,
    bottom_right: Vertex,
    top_right: Vertex,
    top_left: Vertex,
}

impl Face {
    const XPOS: [Vec3; 4] = [
        [1.0, 0.0, 0.0], // bottom-left
        [1.0, 0.0, 1.0], // bottom-right
        [1.0, 1.0, 1.0], // top-right
        [1.0, 1.0, 0.0], // top-left
    ];
    const XNEG: [Vec3; 4] = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 1.0],
    ];
    const YPOS: [Vec3; 4] = [
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];
    const YNEG: [Vec3; 4] = [
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    ];
    const ZPOS: [Vec3; 4] = [
        [1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
    ];
    const ZNEG: [Vec3; 4] = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];

    fn new(axis: Axis, chunk_coord: Coordinate, voxel_coord: Coordinate, color: Vec3) -> Face {
        let base: Vec3 = [
            (chunk_coord.x * CHUNK_SIZE_X + voxel_coord.x) as f32,
            (chunk_coord.y * CHUNK_SIZE_Y + voxel_coord.y) as f32,
            (chunk_coord.z * CHUNK_SIZE_Z + voxel_coord.z) as f32,
        ];
        let face = match axis {
            Axis::Xpos => Face::XPOS,
            Axis::Xneg => Face::XNEG,
            Axis::Ypos => Face::YPOS,
            Axis::Yneg => Face::YNEG,
            Axis::Zpos => Face::ZPOS,
            Axis::Zneg => Face::ZNEG,
        };
        Face {
            bottom_left: Vertex {
                position: add(base, face[0]),
                color,
            },
            bottom_right: Vertex {
                position: add(base, face[1]),
                color,
            },
            top_right: Vertex {
                position: add(base, face[2]),
                color,
            },
            top_left: Vertex {
                position: add(base, face[3]),
                color,
            },
        }
    }
}

/// Stores meshes generated from a chunk. The meshes need to be recomputed if the chunk is updated.
pub struct FaceMeshes {
    /// Voxel faces in the positive x-direction.
    pub x_pos: Mesh,
    /// Voxel faces in the negative x-direction.
    pub x_neg: Mesh,
    /// Voxel faces in the positive y-direction.
    pub y_pos: Mesh,
    /// Voxel faces in the negative y-direction.
    pub y_neg: Mesh,
    /// Voxel faces in the positive z-direction.
    pub z_pos: Mesh,
    /// Voxel faces in the negative z-direction.
    pub z_neg: Mesh,
}

impl FaceMeshes {
    pub fn new(chunk: &Chunk) -> Self {
        Self {
            x_pos: FaceMeshes::build_face_mesh(chunk, Axis::Xpos),
            x_neg: FaceMeshes::build_face_mesh(chunk, Axis::Xneg),
            y_pos: FaceMeshes::build_face_mesh(chunk, Axis::Ypos),
            y_neg: FaceMeshes::build_face_mesh(chunk, Axis::Yneg),
            z_pos: FaceMeshes::build_face_mesh(chunk, Axis::Zpos),
            z_neg: FaceMeshes::build_face_mesh(chunk, Axis::Zneg),
        }
    }

    fn build_face_mesh(chunk: &Chunk, axis: Axis) -> Mesh {
        let next = &axis.as_displacement();
        let faces: Vec<Face> = chunk
            .iter()
            .filter(|(coord, voxel)| {
                !voxel.is_air() && matches!(chunk.get_voxel(coord + next), Some(Voxel::Air) | None)
            })
            .map(|(coord, voxel)| {
                let color: [f32; 3] = match voxel {
                    Voxel::Air => unreachable!(),
                    Voxel::Grass => [0.33, 0.80, 0.46],
                    Voxel::Dirt => [0.35, 0.29, 0.21],
                };
                let weight: f32 = rand::random();
                let color: Vec3 = [color[0] * weight, color[1] * weight, color[2] * weight];
                Face::new(axis, chunk.coord, coord, color)
            })
            .collect();
        let mut vertices: Vec<Vertex> = Vec::with_capacity(faces.len() * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(faces.len() * 6);
        let mut index = 0u32;
        for face in faces {
            vertices.push(face.bottom_left);
            vertices.push(face.bottom_right);
            vertices.push(face.top_right);
            vertices.push(face.top_left);
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
    view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
