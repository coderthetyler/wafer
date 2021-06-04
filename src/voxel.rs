/// Size of chunk in x-direction.
pub const CHUNK_SIZE_X: u32 = 16;

/// Size of chunk in y-direction.
pub const CHUNK_SIZE_Y: u32 = 16;

/// Size of chunk in z-direction.
pub const CHUNK_SIZE_Z: u32 = 16;

/// Total voxel volume of a chunk.
pub const CHUNK_VOLUME: u32 = CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z;

/// Absolute coordinate of a chunk in chunk-space.
#[derive(Copy, Clone)]
pub struct ChunkIndex(i32, i32, i32);

/// Voxel coordinate within a single chunk.
#[derive(Copy, Clone)]
pub struct LocalVoxelIndex(u32, u32, u32);

/// Voxel coordinate within the entire world.
#[derive(Copy, Clone)]
pub struct GlobalVoxelIndex(u32, u32, u32);

impl LocalVoxelIndex {
    /// Index into the list of chunk voxels. Winding order is XZY.
    fn as_index(&self) -> usize {
        (self.0 + self.2 * CHUNK_SIZE_X + self.1 * CHUNK_SIZE_X * CHUNK_SIZE_Z) as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Voxel {
    /// The 'empty' voxel!
    Air,
    Dirt,
    Grass,
}

pub struct Chunk {
    /// Position of chunk in chunk-space.
    pub index: ChunkIndex,
    /// List of all voxels in the chunk. This should always have a size equal to the volume of a chunk.
    voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(index: ChunkIndex) -> Self {
        Self {
            index,
            voxels: Vec::with_capacity(CHUNK_VOLUME as usize),
        }
    }

    /// Get the voxel at the specified index in the chunk.
    pub fn get(&self, index: LocalVoxelIndex) -> Voxel {
        self.voxels[index.as_index()]
    }
}
