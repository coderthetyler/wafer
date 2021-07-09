use super::Application;

pub use self::config::ConfigAction;
pub use self::console::ConsoleAction;
pub use self::scene::SceneAction;
pub use self::window::WindowAction;

mod config;
mod console;
mod scene;
mod window;

pub trait ActionType {
    fn perform(self, app: &mut Application);
}

pub enum Action {
    /// An action scoped to the application state.
    Config(ConfigAction),
    /// An action scoped to the console.
    Console(ConsoleAction),
    /// An action scoped to the scene.
    Scene(SceneAction),
    /// An action scoped to the window.
    Window(WindowAction),
}

impl ActionType for Action {
    /// Perform & consume the action.
    fn perform(self, app: &mut Application) {
        match self {
            Action::Config(action) => {
                action.perform(app);
            }
            Action::Console(action) => {
                action.perform(app);
            }
            Action::Scene(action) => {
                action.perform(app);
            }
            Action::Window(action) => {
                action.perform(app);
            }
        }
    }
}
