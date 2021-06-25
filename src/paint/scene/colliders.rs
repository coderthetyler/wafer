use wgpu::{util::DeviceExt, Device};

use crate::{entity::EntitySystem, geometry::Volume};

/*

# Design: One instanced draw call & model matrix array buffer per collider type.

# To do

1. Init vertex and index buffer for the box volume buffer
2. Create render pipeline
3. Write a vertex shader
4. Add draw calls to WorldPainter

*/

/// All data required to draw collider volumes, if enabled.
pub struct ColliderPainter {
    pipeline: wgpu::RenderPipeline,
    boxes: VolumeBuffer,
}

/// All data used to render a particular volume.
/// The vertex and index data never needs to change, because we use standard size objects.
pub struct VolumeBuffer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instances: VolumeInstanceBuffer,
}

/// Updated each frame with new [`InstanceData`].
/// Many collider volumes are likely moving around between frames.
/// As such, we want to update the model matrices of these volumes.
pub struct VolumeInstanceBuffer {
    buffer: wgpu::Buffer,
    count: usize,
}

impl ColliderPainter {
    pub fn new(&mut self, device: &Device, entities: &EntitySystem) -> Self {
        todo!()
    }

    pub fn update_boxes(
        &mut self,
        device: &Device,
        entities: &EntitySystem,
    ) -> VolumeInstanceBuffer {
        let box_instances: Vec<InstanceData> = entities
            .iter()
            .filter_map(|idx| {
                if let (Some(pos), Some(Volume::Box { x, y, z })) =
                    (entities.positions.get(idx), entities.colliders.get(idx))
                {
                    let instance = InstanceData {
                        model: (cgmath::Matrix4::from_translation([pos.x, pos.y, pos.z].into())
                            * cgmath::Matrix4::from_nonuniform_scale(*x, *y, *z))
                        .into(),
                    };
                    Some(instance)
                } else {
                    None
                }
            })
            .collect();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&box_instances),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let count = box_instances.len();
        VolumeInstanceBuffer { buffer, count }
    }
}

pub struct VertexBuffer {
    pub vertices: wgpu::Buffer,
    pub vertex_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexData {
    position: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    model: [[f32; 4]; 4],
}

impl InstanceData {
    fn new() -> Self {
        Self {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
