use crate::app::Application;

use super::Action;

pub enum AppAction {
    /// Exit the application.
    RequestClose,
}

impl AppAction {
    pub(super) fn perform(self, app: &mut Application) -> Action {
        match self {
            AppAction::RequestClose => {
                app.close_requested = true;
                Action::None
            }
        }
    }
}

impl From<AppAction> for Action {
    fn from(action: AppAction) -> Self {
        Action::App(action)
    }
}
