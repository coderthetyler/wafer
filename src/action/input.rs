use crate::{app::Application, input::InputContext};

use super::Action;

pub enum InputSystemAction {
    /// Pop the current input context.
    PopContext,
    /// Push a new input context & make it active.
    PushContext(InputContext),
}

impl InputSystemAction {
    pub(super) fn perform(self, app: &mut Application) -> Action {
        match self {
            InputSystemAction::PopContext => app.input_system.pop_context(),
            InputSystemAction::PushContext(context) => app.input_system.push_context(context),
        }
    }
}

impl From<InputSystemAction> for Action {
    fn from(action: InputSystemAction) -> Self {
        Action::InputSystem(action)
    }
}
