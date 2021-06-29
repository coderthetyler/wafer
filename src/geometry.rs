use std::ops::Add;

/// A simple mesh used to detect collisions.
pub enum Volume {
    Box { x: f32, y: f32, z: f32 },
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    pub fn zero() -> Self {
        Vec3i { x: 0, y: 0, z: 0 }
    }
}

impl From<[i32; 3]> for Vec3i {
    fn from(array: [i32; 3]) -> Self {
        Vec3i {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vec3f {
    fn from(array: [f32; 3]) -> Self {
        Vec3f {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<[f32; 3]> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: [f32; 3]) -> Vec3f {
        Vec3f {
            x: self.x + rhs[0],
            y: self.y + rhs[1],
            z: self.z + rhs[2],
        }
    }
}
