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
    pub fn perform(self, app: &mut Application) {
        match self {
            Action::None => {}
            Action::Console(action) => {
                action.perform(app);
            }
            Action::Window(action) => {
                action.perform(app);
            }
            Action::App(action) => {
                action.perform(app);
            }
        }
    }
}
