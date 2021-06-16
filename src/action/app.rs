use crate::app::Application;

use super::{Action, ActionType};

pub enum AppAction {
    /// Exit the application.
    RequestClose,
}

impl ActionType for AppAction {
    fn perform(self, app: &mut Application) {
        match self {
            AppAction::RequestClose => {
                app.close_requested = true;
            }
        }
    }
}

impl From<AppAction> for Action {
    fn from(action: AppAction) -> Self {
        Action::App(action)
    }
}
