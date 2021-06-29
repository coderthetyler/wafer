use crate::{entity::EntityPool, geometry::Vec3f, time::Frame};

pub type Spin = Vec3f;
pub type Velocity = Vec3f;

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, ecs: &mut EntityPool) {
        let delta = frame.delta.as_f32();
        for entity in ecs.pool.iter() {
            if let (Some(pos), Some(vel)) = (ecs.position.get_mut(entity), ecs.velocity.get(entity))
            {
                pos.x += vel.x * delta;
                pos.y += vel.y * delta;
                pos.z += vel.z * delta;
            }

            if let (Some(rot), Some(vel)) = (ecs.rotation.get_mut(entity), ecs.spin.get(entity)) {
                rot.x += vel.x * delta;
                rot.y += vel.y * delta;
                rot.z += vel.z * delta;
            }
        }
    }
}
