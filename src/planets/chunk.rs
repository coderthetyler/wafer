use std::ops::Add;

use crate::{
    draw::{ColorVertex, Mesh},
    geometry::Vec3f,
};

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
                            [color[0] * weight, color[1] * weight, color[2] * weight].into();
                        let base: Vec3f = [
                            (self.coord.x * CHUNK_SIZE_X + coord.x) as f32,
                            (self.coord.y * CHUNK_SIZE_Y + coord.y) as f32,
                            (self.coord.z * CHUNK_SIZE_Z + coord.z) as f32,
                        ]
                        .into();
                        [
                            ColorVertex {
                                position: base + unit_face[0],
                                color,
                            },
                            ColorVertex {
                                position: base + unit_face[1],
                                color,
                            },
                            ColorVertex {
                                position: base + unit_face[2],
                                color,
                            },
                            ColorVertex {
                                position: base + unit_face[3],
                                color,
                            },
                        ]
                    })
            })
            .flatten()
            .collect();

        let mut vertices: Vec<ColorVertex> = Vec::with_capacity(faces.len() * 4);
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
