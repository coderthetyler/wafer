use crate::app::Application;

use super::{Action, ActionType};

pub enum AppAction {
    /// Show & hide the debugging overlay.
    ToggleDebugOverlay,
    /// Exit the application.
    RequestClose,
}

impl ActionType for AppAction {
    fn perform(self, app: &mut Application) {
        match self {
            AppAction::RequestClose => {
                app.close_requested = true;
            }
            AppAction::ToggleDebugOverlay => {
                app.paint_system.overlay.show_debug_overlay ^= true;
            }
        }
    }
}

impl From<AppAction> for Action {
    fn from(action: AppAction) -> Self {
        Action::App(action)
    }
}
