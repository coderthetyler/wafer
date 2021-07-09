use crate::{entity::Ecs, frame::Frame};

pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, _frame: &Frame, _ecs: &mut Ecs) {}
}
