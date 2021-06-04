use crate::voxel::Chunk;
use crate::voxel::ChunkIndex;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorVertex {
    position: [f32; 3],
    color: [f32; 3],
}

/// Stores a mesh as a list of vertices and an index buffer.
struct Mesh {
    vertices: Vec<ColorVertex>,
    indices: Vec<u32>,
}

/// Stores meshes generated from a chunk. The meshes need to be recomputed if the chunk is updated.
pub struct ChunkMeshes {
    /// Index of the chunk these meshes are constructed from.
    index: ChunkIndex,
    /// Voxel faces in the positive x-direction.
    x_pos: Mesh,
    /// Voxel faces in the negative x-direction.
    x_neg: Mesh,
    /// Voxel faces in the positive y-direction.
    y_pos: Mesh,
    /// Voxel faces in the negative y-direction.
    y_neg: Mesh,
    /// Voxel faces in the positive z-direction.
    z_pos: Mesh,
    /// Voxel faces in the negative z-direction.
    z_neg: Mesh,
}

#[derive(Copy, Clone)]
enum Direction {
    Xpos,
    Xneg,
    Ypos,
    Yneg,
    Zpos,
    Zneg,
}

impl ChunkMeshes {
    fn build_face_mesh(chunk: &Chunk, direction: Direction) -> Mesh {
        // Maybe it's better to build all face meshes at once?
        // Is there an ideal voxel indexing strategy? A 'linear' winding order may not be ideal.
        todo!("Implement me!")
    }
}

impl Chunk {
    pub fn build_meshes(&self) -> ChunkMeshes {
        ChunkMeshes {
            index: self.index,
            x_pos: ChunkMeshes::build_face_mesh(self, Direction::Xpos),
            x_neg: ChunkMeshes::build_face_mesh(self, Direction::Xneg),
            y_pos: ChunkMeshes::build_face_mesh(self, Direction::Ypos),
            y_neg: ChunkMeshes::build_face_mesh(self, Direction::Yneg),
            z_pos: ChunkMeshes::build_face_mesh(self, Direction::Zpos),
            z_neg: ChunkMeshes::build_face_mesh(self, Direction::Zneg),
        }
    }
}
