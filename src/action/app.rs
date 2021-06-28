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
                app.close_requested = true;
            }
            AppAction::ToggleDebugOverlay => {
                app.state.hide_debug_overlay ^= true;
            }
            AppAction::ToglePaintColliderVolumes => {
                app.state.show_collider_volumes ^= true;
            }
        }
    }
}

impl From<AppAction> for Action {
    fn from(action: AppAction) -> Self {
        Action::App(action)
    }
}
