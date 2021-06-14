use winit::{event::Event, window::WindowId};

use crate::{entity::EntitySystem, time::Seconds};

pub use self::camera::CameraInputContext;

mod camera;

pub struct InputSystem {
    context_stack: Vec<InputContext>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            context_stack: vec![],
        }
    }

    /// Make the given `context` the top-most selected input context.
    pub fn push_context(&mut self, context: InputContext) {
        self.context_stack.push(context);
    }

    /// Remove the topmost input context from the priority stack, if any.
    pub fn pop_context(&mut self) -> Option<InputContext> {
        self.context_stack.pop()
    }

    /// Update the active input context, if any.
    pub fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        if let Some(context) = self.context_stack.last_mut() {
            context.update(entities, delta);
        }
    }

    /// Pass the `event` to the active input context, if any.
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> bool {
        if let Some(context) = self.context_stack.last_mut() {
            context.receive_event(windowid, event)
        } else {
            false
        }
    }
}

/// Enum polymorphism!
pub enum InputContext {
    Camera(CameraInputContext),
}

impl InputContext {
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> bool {
        match self {
            InputContext::Camera(context) => context.receive_event(windowid, event),
        }
    }

    pub fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        match self {
            InputContext::Camera(context) => context.update(entities, delta),
        }
    }
}

impl From<CameraInputContext> for InputContext {
    fn from(context: CameraInputContext) -> Self {
        InputContext::Camera(context)
    }
}

impl From<CameraInputContext> for Option<InputContext> {
    fn from(context: CameraInputContext) -> Self {
        Some(InputContext::Camera(context))
    }
}
