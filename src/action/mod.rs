use super::Application;

pub use self::console::ConsoleAction;
pub use self::input::InputSystemAction;

mod console;
mod input;

pub enum Action {
    /// Indicates the absence of an action, semantically similar to [`Option::None`].
    /// This is intended to indicate that an action producer was unable to produce one.
    None,
    /// Action that performs no operation.
    NoOp,
    /// An action scoped to the input system.
    InputSystem(InputSystemAction),
    /// An action scoped to the console.
    Console(ConsoleAction),
}

impl Action {
    pub fn perform(self, app: &mut Application) {
        match self {
            Action::None => {}
            Action::NoOp => {}
            Action::InputSystem(action) => action.perform(app),
            Action::Console(action) => action.perform(app),
        }
    }
}
