use winit::{event::Event, window::WindowId};

use crate::action::Action;
use crate::entity::{EntityComponents, EntityPool};
use crate::time::Frame;

use self::console::ConsoleInputContext;
use self::puppet::PuppetInputContext;

pub mod console;
pub mod puppet;

pub struct InputReceiver {
    context_stack: Vec<InputContext>,
}

impl InputReceiver {
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
    pub fn update(
        &mut self,
        frame: &Frame,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
    ) {
        if let Some(context) = self.context_stack.last_mut() {
            context.update(frame, entities, components);
        }
    }

    /// Pass the `event` to the active input context, if any.
    /// Returns `true` if the context consumed the event.
    pub fn receive_event(
        &mut self,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
        windowid: &WindowId,
        event: &Event<()>,
    ) -> EventAction {
        if let Some(context) = self.context_stack.last_mut() {
            context.receive_event(entities, components, windowid, event)
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

pub enum InputContext {
    Scene(PuppetInputContext),
    Console(ConsoleInputContext),
}

impl InputContext {
    fn on_active(&mut self) -> Option<Action> {
        match self {
            InputContext::Scene(context) => context.on_active(),
            InputContext::Console(context) => context.on_active(),
        }
    }

    fn receive_event(
        &mut self,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
        windowid: &WindowId,
        event: &Event<()>,
    ) -> EventAction {
        match self {
            InputContext::Scene(context) => {
                context.receive_event(entities, components, windowid, event)
            }
            InputContext::Console(context) => {
                context.receive_event(entities, components, windowid, event)
            }
        }
    }

    fn update(&self, frame: &Frame, entities: &mut EntityPool, components: &mut EntityComponents) {
        match self {
            InputContext::Scene(context) => context.update(frame, entities, components),
            InputContext::Console(context) => context.update(frame, entities, components),
        }
    }
}

impl From<PuppetInputContext> for InputContext {
    fn from(context: PuppetInputContext) -> Self {
        InputContext::Scene(context)
    }
}

impl From<ConsoleInputContext> for InputContext {
    fn from(context: ConsoleInputContext) -> Self {
        InputContext::Console(context)
    }
}
