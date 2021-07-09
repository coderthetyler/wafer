use crate::{entity::Ecs, frame::Frame};

pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, frame: &Frame, ecs: &mut Ecs) {}
}
