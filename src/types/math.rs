use std::ops::Add;

pub type Spin = Vec3f;
pub type Velocity = Vec3f;

/// A simple mesh used to detect collisions.
pub enum Volume {
    Box { x: f32, y: f32, z: f32 },
}

/// Represents a rectangle in 2-space.
#[derive(Clone, Copy, Debug, Default)]
pub struct Extent {
    pub width: f32,
    pub height: f32,
}

/// Represents a point in 2-space.
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Represents a position in 3-space.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rotation {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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

/// Weight falloff.
pub enum Falloff {
    /// Weight is divided each iteration.
    Geometric(f32),
}

impl Falloff {
    pub fn apply(&self, weight: f32) -> f32 {
        match self {
            Falloff::Geometric(falloff) => weight / falloff,
        }
    }
}

/// Aspect ratio of a rectangle.
pub struct AspectRatio(f32);

impl From<(f32, f32)> for AspectRatio {
    fn from((width, height): (f32, f32)) -> Self {
        AspectRatio(width / height)
    }
}

impl From<f32> for AspectRatio {
    fn from(aspect_ratio: f32) -> Self {
        AspectRatio(aspect_ratio)
    }
}

impl From<AspectRatio> for f32 {
    fn from(aspect_ratio: AspectRatio) -> Self {
        aspect_ratio.0
    }
}
