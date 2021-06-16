use super::Application;

use self::app::AppAction;
pub use self::console::ConsoleAction;
pub use self::window::WindowAction;

mod app;
mod console;
mod window;

pub enum Action {
    /// An action that does nothing when performed.
    None,
    /// An action scoped to the console.
    Console(ConsoleAction),
    /// An action scoped to the window.
    Window(WindowAction),
    /// An action scope to the application.
    App(AppAction),
}

impl Action {
    /// Perform & consume the action.
    /// Sequenced actions are performed in order.
    pub fn perform(self, app: &mut Application) {
        let mut next_action = self;
        loop {
            match next_action {
                Action::None => break,
                Action::Console(action) => {
                    next_action = action.perform(app);
                }
                Action::Window(action) => {
                    next_action = action.perform(app);
                }
                Action::App(action) => {
                    next_action = action.perform(app);
                }
            }
        }
    }
}
