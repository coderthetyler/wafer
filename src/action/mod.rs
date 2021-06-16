use super::Application;

pub use self::console::ConsoleAction;
pub use self::input::InputSystemAction;
pub use self::window::WindowAction;

mod console;
mod input;
mod window;

pub enum Action {
    /// An action that does nothing when performed.
    None,
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
        let mut next_action = self;
        loop {
            match next_action {
                Action::None => break,
                Action::InputSystem(action) => {
                    next_action = action.perform(app);
                }
                Action::Console(action) => {
                    next_action = action.perform(app);
                }
                Action::Window(action) => {
                    next_action = action.perform(app);
                }
            }
        }
    }
}
