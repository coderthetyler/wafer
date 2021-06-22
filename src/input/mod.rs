use winit::{event::Event, window::WindowId};

use crate::action::Action;
use crate::entity::EntitySystem;
use crate::time::Frame;

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
    pub fn push_context(&mut self, context: InputContext) -> Option<Action> {
        let mut context = context;
        let action = context.on_active();
        self.context_stack.push(context);
        action
    }

    /// Remove the topmost input context from the priority stack, if any.
    pub fn pop_context(&mut self) -> Option<Action> {
        self.context_stack.pop();
        if let Some(context) = self.context_stack.last_mut() {
            context.on_active()
        } else {
            None
        }
    }

    /// Update the active input context, if any.
    pub fn update(&mut self, frame: &Frame, entities: &mut EntitySystem) {
        if let Some(context) = self.context_stack.last_mut() {
            context.update(frame, entities);
        }
    }

    /// Pass the `event` to the active input context, if any.
    /// Returns `true` if the context consumed the event.
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        if let Some(context) = self.context_stack.last_mut() {
            context.receive_event(windowid, event)
        } else {
            EventAction::Unconsumed
        }
    }
}

pub enum EventAction {
    /// The input event was unprocessed by the context.
    Unconsumed,
    /// The input event was consumed by the context but produced no action.
    Consumed,
    /// The input event was consumed by the context and produced an action.
    React(Action),
}

impl From<Action> for EventAction {
    fn from(action: Action) -> Self {
        EventAction::React(action)
    }
}

pub trait InputContextType {
    fn on_active(&mut self) -> Option<Action>;
    fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction;
    fn update(&mut self, frame: &Frame, entities: &mut EntitySystem);
}

pub enum InputContext {
    Camera(CameraInputContext),
    Console(ConsoleInputContext),
}

impl InputContextType for InputContext {
    fn on_active(&mut self) -> Option<Action> {
        match self {
            InputContext::Camera(context) => context.on_active(),
            InputContext::Console(context) => context.on_active(),
        }
    }

    fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        match self {
            InputContext::Camera(context) => context.receive_event(windowid, event),
            InputContext::Console(context) => context.receive_event(windowid, event),
        }
    }

    fn update(&mut self, frame: &Frame, entities: &mut EntitySystem) {
        match self {
            InputContext::Camera(context) => context.update(frame, entities),
            InputContext::Console(context) => context.update(frame, entities),
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
