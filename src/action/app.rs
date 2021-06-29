use crate::app::Application;

use super::{Action, ActionType};

pub enum AppAction {
    /// Show & hide collider volume wireframes.
    ToglePaintColliderVolumes,
    /// Show & hide the debugging overlay.
    ToggleDebugOverlay,
    /// Exit the application.
    RequestClose,
}

impl ActionType for AppAction {
    fn perform(self, app: &mut Application) {
        match self {
            AppAction::RequestClose => {
                app.config.should_exit = true;
            }
            AppAction::ToggleDebugOverlay => {
                app.config.hide_debug_overlay ^= true;
            }
            AppAction::ToglePaintColliderVolumes => {
                app.config.show_collider_volumes ^= true;
            }
        }
    }
}

impl From<AppAction> for Action {
    fn from(action: AppAction) -> Self {
        Action::App(action)
    }
}
