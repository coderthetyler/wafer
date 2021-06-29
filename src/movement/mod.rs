use crate::{entity::EntitySystem, time::Frame};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, entity_system: &mut EntitySystem) {
        let delta = frame.delta.as_f32();
        for entity in entity_system.entities.iter() {
            if let (Some(pos), Some(vel)) = (
                entity_system.positions.get_mut(entity),
                entity_system.velocities.get(entity),
            ) {
                pos.x += vel.x * delta;
                pos.y += vel.y * delta;
                pos.z += vel.z * delta;
            }

            if let (Some(rot), Some(vel)) = (
                entity_system.rotations.get_mut(entity),
                entity_system.angular_velocities.get(entity),
            ) {
                rot.x += vel.x * delta;
                rot.y += vel.y * delta;
                rot.z += vel.z * delta;
            }
        }
    }
}
