use std::ops::Add;

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

#[derive(Clone, Copy, Debug)]
pub enum Voxel {
    Air,
    Grass,
    Dirt,
}

impl Voxel {
    pub fn is_air(&self) -> bool {
        // match self {
        //     Voxel::Air => true,
        //     _ => false,
        // }
        // matches!(*self, Voxel::Air)
        matches!(*self, Voxel::Air)
    }
}

pub struct Chunk {
    pub coord: Coordinate,
    voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Chunk {
            coord: Coordinate { x, y, z },
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

#[derive(Clone, Copy)]
pub enum Axis {
    Xpos,
    Xneg,
    Ypos,
    Yneg,
    Zpos,
    Zneg,
}

#[derive(Debug)]
pub struct Displacement {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Displacement {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Displacement { x, y, z }
    }
}

impl Axis {
    pub fn as_displacement(&self) -> Displacement {
        match self {
            Axis::Xpos => Displacement { x: 1, y: 0, z: 0 },
            Axis::Xneg => Displacement { x: -1, y: 0, z: 0 },
            Axis::Ypos => Displacement { x: 0, y: 1, z: 0 },
            Axis::Yneg => Displacement { x: 0, y: -1, z: 0 },
            Axis::Zpos => Displacement { x: 0, y: 0, z: 1 },
            Axis::Zneg => Displacement { x: 0, y: 0, z: -1 },
        }
    }
}

impl Add<&Displacement> for &Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &Displacement) -> Coordinate {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
