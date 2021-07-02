use winit::{event::Event, window::WindowId};

use crate::action::{Action, ConsoleAction, SceneAction};

pub use self::console::ConsoleInputContext;
pub use self::scene::SceneInputContext;
pub use self::scene::SceneInputState;

mod console;
mod scene;

pub struct EventInterpreter {
    context_stack: Vec<InputContext>,
}

impl EventInterpreter {
    pub fn new() -> Self {
        Self {
            context_stack: vec![],
        }
    }

    /// Make the given `context` the top-most selected input context.
    pub fn push_context<C>(&mut self, context: C) -> Option<Action>
    where
        C: Into<InputContext>,
    {
        let mut context = context.into();
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

    /// Pass the `event` to the active input context, if any.
    /// Returns `true` if the context consumed the event.
    pub fn consume(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        if let Some(context) = self.context_stack.last_mut() {
            context.interpret(windowid, event)
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

impl From<ConsoleAction> for EventAction {
    fn from(action: ConsoleAction) -> Self {
        EventAction::React(Action::Console(action))
    }
}

impl From<SceneAction> for EventAction {
    fn from(action: SceneAction) -> Self {
        EventAction::React(Action::Scene(action))
    }
}

/// Each input context has its own bindings.
/// Contexts may safely bind different actions to the same events.
pub enum InputContext {
    Scene(SceneInputContext),
    Console(ConsoleInputContext),
}

impl InputContext {
    fn on_active(&mut self) -> Option<Action> {
        match self {
            InputContext::Scene(context) => context.on_active(),
            InputContext::Console(context) => context.on_active(),
        }
    }

    fn interpret(&mut self, wid: &WindowId, event: &Event<()>) -> EventAction {
        match self {
            InputContext::Scene(context) => context.interpret(wid, event),
            InputContext::Console(context) => context.interpret(wid, event),
        }
    }
}

impl From<SceneInputContext> for InputContext {
    fn from(context: SceneInputContext) -> Self {
        InputContext::Scene(context)
    }
}

impl From<ConsoleInputContext> for InputContext {
    fn from(context: ConsoleInputContext) -> Self {
        InputContext::Console(context)
    }
}
