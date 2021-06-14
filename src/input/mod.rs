use winit::event::{VirtualKeyCode, WindowEvent};
use winit::{event::Event, window::WindowId};

use crate::console::Console;
use crate::{entity::EntitySystem, time::Seconds};

pub use self::camera::CameraInputContext;
pub use self::console::ConsoleInputContext;

mod camera;
mod console;

pub enum ContextAction {
    Pop,
    ConsumedEvent,
    UnconsumedEvent,
}

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
    /// Returns `true` if the context consumed the event.
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> bool {
        if self.should_open_console(windowid, event) {
            // self.push_context(ConsoleInputContext::new(&self.console).into());
            return true;
        }
        if let Some(context) = self.context_stack.last_mut() {
            match context.receive_event(windowid, event) {
                ContextAction::ConsumedEvent => true,
                ContextAction::Pop => {
                    self.pop_context();
                    true
                }
                ContextAction::UnconsumedEvent => false,
            }
        } else {
            false
        }
    }

    fn should_open_console(&self, windowid: &WindowId, event: &Event<()>) -> bool {
        if let Event::WindowEvent { window_id, event } = event {
            if windowid == window_id {
                if let WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(VirtualKeyCode::T) = input.virtual_keycode {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub enum InputContext {
    Camera(CameraInputContext),
    Console(ConsoleInputContext),
}

impl InputContext {
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> ContextAction {
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
