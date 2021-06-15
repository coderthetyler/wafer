use winit::{event::Event, window::WindowId};

use crate::action::Action;
use crate::{entity::EntitySystem, time::Seconds};

pub use self::camera::CameraInputContext;
pub use self::console::ConsoleInputContext;

mod camera;
mod console;

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
    pub fn push_context(&mut self, context: InputContext) -> Action {
        let mut context = context;
        let action = context.on_active();
        self.context_stack.push(context);
        action
    }

    /// Remove the topmost input context from the priority stack, if any.
    pub fn pop_context(&mut self) -> Action {
        self.context_stack.pop();
        if let Some(context) = self.context_stack.last_mut() {
            context.on_active()
        } else {
            Action::None
        }
    }

    /// Update the active input context, if any.
    pub fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        if let Some(context) = self.context_stack.last_mut() {
            context.update(entities, delta);
        }
    }

    /// Pass the `event` to the active input context, if any.
    /// Returns `true` if the context consumed the event.
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> Action {
        if let Some(context) = self.context_stack.last_mut() {
            context.receive_event(windowid, event)
        } else {
            Action::None
        }
    }
}

pub enum InputContext {
    Camera(CameraInputContext),
    Console(ConsoleInputContext),
}

impl InputContext {
    pub fn on_active(&mut self) -> Action {
        match self {
            InputContext::Camera(context) => context.on_active(),
            InputContext::Console(context) => context.on_active(),
        }
    }

    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> Action {
        match self {
            InputContext::Camera(context) => context.receive_event(windowid, event),
            InputContext::Console(context) => context.receive_event(windowid, event),
        }
    }

    pub fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        match self {
            InputContext::Camera(context) => context.update(entities, delta),
            InputContext::Console(context) => context.update(entities, delta),
        }
    }
}

impl From<CameraInputContext> for InputContext {
    fn from(context: CameraInputContext) -> Self {
        InputContext::Camera(context)
    }
}

impl From<ConsoleInputContext> for InputContext {
    fn from(context: ConsoleInputContext) -> Self {
        InputContext::Console(context)
    }
}
