use crate::geometry::Vec3f;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: Vec3f,
    pub color: Vec3f,
}

pub struct Mesh {
    pub vertices: Vec<ColorVertex>,
    pub indices: Vec<u32>,
}
