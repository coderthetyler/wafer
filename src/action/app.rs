use crate::app::Application;

use super::{Action, ActionType};

pub enum ConfigAction {
    /// Show & hide collider volume wireframes.
    ToglePaintColliderVolumes,
    /// Show & hide the debugging overlay.
    ToggleDebugOverlay,
    /// Exit the application.
    RequestClose,
}

impl ActionType for ConfigAction {
    fn perform(self, app: &mut Application) {
        match self {
            ConfigAction::RequestClose => {
                app.config.should_exit = true;
            }
            ConfigAction::ToggleDebugOverlay => {
                app.config.hide_debug_overlay ^= true;
            }
            ConfigAction::ToglePaintColliderVolumes => {
                app.config.show_collider_volumes ^= true;
            }
        }
    }
}

impl From<ConfigAction> for Action {
    fn from(action: ConfigAction) -> Self {
        Action::Config(action)
    }
}
