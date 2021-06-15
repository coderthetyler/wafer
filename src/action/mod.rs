use super::Application;

pub use self::console::ConsoleAction;
pub use self::input::InputSystemAction;
pub use self::window::WindowAction;

mod console;
mod input;
mod window;

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
    /// An action scoped to the window.
    Window(WindowAction),
}

impl Action {
    /// Perform & consume the action.
    /// Sequenced actions are performed in order.
    pub fn perform(self, app: &mut Application) {
        let mut top_level_action: Action = self;
        loop {
            match top_level_action {
                Action::None => break,
                Action::NoOp => break,
                Action::InputSystem(action) => {
                    top_level_action = action.perform(app);
                }
                Action::Console(action) => {
                    top_level_action = action.perform(app);
                }
                Action::Window(action) => {
                    top_level_action = action.perform(app);
                }
            }
        }
    }
}
