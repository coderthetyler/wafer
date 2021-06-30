use winit::{event::Event, window::WindowId};

use crate::{
    entity::{Entity, EntityComponents, EntityDelta, EntityPool},
    input::EventAction,
    payload::Payload,
    time::Frame,
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
    /// Update the puppet at the end of the frame.
    pub fn post_update(&mut self, frame: &Frame) {
        match self {
            Puppet::FreeCamera(puppet) => puppet.post_update(frame),
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

    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        match self {
            Puppet::FreeCamera(controller) => controller.receive_event(windowid, event),
        }
    }
}

pub struct PuppetSystem {}

impl PuppetSystem {
    pub fn update(
        &self,
        frame: &Frame,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
    ) {
        for entity in entities.iter() {
            if let Some(puppet) = components.puppet.get(entity) {
                puppet
                    .gen_deltas(frame, entity, components)
                    .iter()
                    .for_each(|delta| delta.apply_to(entity, components));
            }
        }
        for entity in entities.iter() {
            if let Some(puppet) = components.puppet.get_mut(entity) {
                puppet.post_update(frame);
            }
        }
    }
}
