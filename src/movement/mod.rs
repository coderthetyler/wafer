use crate::{entity::Ecs, frame::Frame};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, ecs: &mut Ecs) {
        let delta = frame.delta.as_f32();
        for entity in ecs.pool.iter() {
            if let (Some(pos), Some(vel)) = (
                ecs.comps.position.get_mut(entity),
                ecs.comps.velocity.get(entity),
            ) {
                pos.x += vel.x * delta;
                pos.y += vel.y * delta;
                pos.z += vel.z * delta;
            }

            if let (Some(rot), Some(vel)) = (
                ecs.comps.rotation.get_mut(entity),
                ecs.comps.spin.get(entity),
            ) {
                rot.pitch += vel.x * delta;
                rot.yaw += vel.y * delta;
                rot.roll += vel.z * delta;
            }
        }
    }
}
