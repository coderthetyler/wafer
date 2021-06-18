use crate::app::Application;

use super::Action;

pub enum WindowAction {
    /// Grab cursor & make it invisible.
    GrabCursor,
    /// Ungrab cursor & make it visible.
    UngrabCursor,
}

impl WindowAction {
    pub fn perform(self, app: &mut Application) {
        match self {
            WindowAction::GrabCursor => {
                app.window.set_cursor_grab(true).unwrap();
                app.window.set_cursor_visible(false);
            }
            WindowAction::UngrabCursor => {
                app.window.set_cursor_grab(false).unwrap();
                app.window.set_cursor_visible(true);
            }
        }
    }
}

impl From<WindowAction> for Action {
    fn from(action: WindowAction) -> Self {
        Action::Window(action)
    }
}
