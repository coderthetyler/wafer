use std::ops::Add;

use super::voxel::Voxel;

/// Size of chunk in x-direction.
pub const CHUNK_SIZE_X: i32 = 16;

/// Size of chunk in y-direction.
pub const CHUNK_SIZE_Y: i32 = 16;

/// Size of chunk in z-direction.
pub const CHUNK_SIZE_Z: i32 = 16;

/// Total voxel volume of a chunk.
pub const CHUNK_VOLUME: usize = (CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z) as usize;

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate {
    pub fn as_index(&self) -> usize {
        (self.x + (self.z * CHUNK_SIZE_X) + (self.y * CHUNK_SIZE_X * CHUNK_SIZE_Z)) as usize
    }

    pub fn zero() -> Self {
        Coordinate { x: 0, y: 0, z: 0 }
    }

    pub fn get_next_local_voxel_coord(&self) -> Option<Coordinate> {
        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;
        x += 1;
        if x >= CHUNK_SIZE_X {
            x -= CHUNK_SIZE_X;
            z += 1;
            if z >= CHUNK_SIZE_Z {
                z -= CHUNK_SIZE_Z;
                y += 1;
                if y >= CHUNK_SIZE_Y {
                    return None;
                }
            }
        }
        Some(Self { x, y, z })
    }
}

impl From<[i32; 3]> for Coordinate {
    fn from(array: [i32; 3]) -> Self {
        Coordinate {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Add<[i32; 3]> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: [i32; 3]) -> Coordinate {
        Coordinate {
            x: self.x + rhs[0],
            y: self.y + rhs[1],
            z: self.z + rhs[2],
        }
    }
}

impl Voxel {
    pub fn is_air(&self) -> bool {
        matches!(*self, Voxel::Air)
    }
}

pub struct Chunk {
    pub coord: Coordinate,
    voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(coord: Coordinate) -> Self {
        Chunk {
            coord,
            voxels: vec![Voxel::Air; CHUNK_VOLUME],
        }
    }

    pub fn randomize(&mut self) {
        self.voxels.iter_mut().for_each(|voxel| {
            let value: f32 = rand::random();
            if value < 0.05 {
                *voxel = Voxel::Dirt
            } else if value < 0.1 {
                *voxel = Voxel::Grass
            }
        });
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter {
            chunk: self,
            coord: Some(Coordinate::zero()),
        }
    }

    pub fn get_voxel(&self, coord: Coordinate) -> Option<&Voxel> {
        if coord.x < 0
            || coord.x >= CHUNK_SIZE_X
            || coord.y < 0
            || coord.y >= CHUNK_SIZE_Y
            || coord.z < 0
            || coord.z >= CHUNK_SIZE_Z
        {
            None
        } else {
            Some(&self.voxels[coord.as_index()])
        }
    }
}

pub struct ChunkIter<'chunk> {
    chunk: &'chunk Chunk,
    coord: Option<Coordinate>,
}

impl<'chunk> Iterator for ChunkIter<'chunk> {
    type Item = (Coordinate, &'chunk Voxel);

    fn next(&mut self) -> Option<(Coordinate, &'chunk Voxel)> {
        let coord = self.coord?;
        let voxel = &self.chunk.voxels[coord.as_index()];
        self.coord = coord.get_next_local_voxel_coord();
        Some((coord, voxel))
    }
}

impl Add<&[i32; 3]> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &[i32; 3]) -> Coordinate {
        Coordinate {
            x: self.x + rhs[0],
            y: self.y + rhs[1],
            z: self.z + rhs[2],
        }
    }
}
