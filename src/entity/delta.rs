use crate::types::Rotation;

use super::{Entity, EntityComponents};

/// An entity delta is a message passing primitive.
/// If a delta is not applicable to an entity, then it is a no-op when applied to that entity.
///
/// Deltas are used to mutate the ECS without needing to pass mutable references around.
/// This type should be viewed as a repertoire of all possible ways of mutating an entity.
#[derive(Clone, Copy)]
pub enum EntityDelta {
    Translate(f32, f32, f32),
    SetRotation(Rotation),
}

impl EntityDelta {
    /// Apply the delta to a target `entity`.
    pub fn apply_to(self, entity: Entity, components: &mut EntityComponents) {
        match self {
            EntityDelta::Translate(x, y, z) => {
                if let Some(pos) = components.position.get_mut(entity) {
                    pos.x += x;
                    pos.y += y;
                    pos.z += z;
                }
            }
            EntityDelta::SetRotation(new_rot) => {
                if let Some(rot) = components.rotation.get_mut(entity) {
                    rot.pitch = new_rot.pitch;
                    rot.yaw = new_rot.yaw;
                    rot.roll = new_rot.roll;
                }
            }
        }
    }
}
