use crate::{
    entity::{EntityComponents, EntityPool},
    frame::Frame,
};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, entities: &EntityPool, components: &mut EntityComponents) {
        let delta = frame.delta.as_f32();
        for entity in entities.iter() {
            if let (Some(pos), Some(vel)) = (
                components.position.get_mut(entity),
                components.velocity.get(entity),
            ) {
                pos.x += vel.x * delta;
                pos.y += vel.y * delta;
                pos.z += vel.z * delta;
            }

            if let (Some(rot), Some(vel)) = (
                components.rotation.get_mut(entity),
                components.spin.get(entity),
            ) {
                rot.pitch += vel.x * delta;
                rot.yaw += vel.y * delta;
                rot.roll += vel.z * delta;
            }
        }
    }
}
