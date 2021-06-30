use crate::{
    entity::{Entity, EntityComponents, EntityDelta, EntityPool},
    frame::Frame,
    payload::Payload,
};

use self::camera::FreeCameraPuppet;

pub mod camera;

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
    pub fn gen_deltas(
        &self,
        frame: &Frame,
        entity: Entity,
        components: &EntityComponents,
    ) -> Payload<EntityDelta> {
        match self {
            Puppet::FreeCamera(puppet) => puppet.gen_deltas(frame, entity, components),
        }
    }
}

pub struct PuppetSystem {}

impl PuppetSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &self,
        frame: &Frame,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
    ) {
        for entity in entities.iter() {
            if let Some(puppet) = components.puppet.get_mut(entity) {
                puppet.pre_update(frame);
            }
        }
        for entity in entities.iter() {
            if let Some(puppet) = components.puppet.get(entity) {
                puppet
                    .gen_deltas(frame, entity, components)
                    .iter()
                    .for_each(|delta| delta.apply_to(entity, components));
            }
        }
    }
}
