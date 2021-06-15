use crate::{app::Application, input::InputContext};

pub enum InputSystemAction {
    /// Pop the current input context.
    PopContext,
    /// Push a new input context & make it active.
    PushContext(InputContext),
}

impl InputSystemAction {
    pub fn perform(self, app: &mut Application) {
        match self {
            InputSystemAction::PopContext => {
                app.input_system.pop_context();
            }
            InputSystemAction::PushContext(context) => {
                app.input_system.push_context(context);
            }
        }
    }
}
