mod camera;

use crate::{
    entity::{Ecs, Entity, EntityDelta},
    frame::Frame,
    types::Payload,
};

pub use self::camera::FreeCameraPuppet;

/// A puppet describes the high-level faculties for piloting an entity.
/// It does not provide any "mind" that calls those faculties.
/// Variants could include a free-camera, in-world locomotion, and more.
pub enum Puppet {
    FreeCamera(FreeCameraPuppet),
}

impl Puppet {
    /// Update the puppet at the start of the frame.
    pub fn pre_update(&mut self, frame: &Frame) {
        match self {
            Puppet::FreeCamera(puppet) => puppet.pre_update(frame),
        }
    }

    /// Generate a set of deltas to be applied to the `entity`.
    pub fn gen_deltas(&self, frame: &Frame, entity: Entity, ecs: &Ecs) -> Payload<EntityDelta> {
        match self {
            Puppet::FreeCamera(puppet) => puppet.gen_deltas(frame, entity, ecs),
        }
    }
}

pub struct PuppetSystem {}

impl PuppetSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, ecs: &mut Ecs) {
        for entity in ecs.pool.iter() {
            if let Some(puppet) = ecs.comps.puppet.get_mut(entity) {
                puppet.pre_update(frame);
            }
            if let Some(puppet) = ecs.comps.puppet.get(entity) {
                for delta in puppet.gen_deltas(frame, entity, ecs).iter() {
                    delta.apply_to(entity, &mut ecs.comps);
                }
            }
        }
    }
}
