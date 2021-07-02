use crate::{
    entity::{EntityComponents, EntityPool},
    frame::Frame,
};

pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, entities: &EntityPool, components: &mut EntityComponents) {}
}
