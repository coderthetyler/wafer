use ascii::AsciiChar;

use crate::app::Application;

use super::Action;

pub enum ConsoleAction {
    /// Insert a character into the console.
    Insert(AsciiChar),
    /// Remove a character from the console at the cursor position.
    Backspace,
    /// Submit the contents of the console & perform the resulting action.
    Submit,
    /// Navigate backwards in the console submission history.
    NavigateBackwards,
    /// Navigate forwards in the console submission history.
    NavigateForwards,
    /// Shift the cursor one character to the left.
    ShiftLeft,
    /// Shift the cursor one character to the right.
    ShiftRight,
    /// Shift the cursor to the beginning of the line.
    ShiftHome,
    /// Shift the cursor to the end of the line.
    ShiftEnd,
}

impl ConsoleAction {
    pub(super) fn perform(self, app: &mut Application) -> Action {
        match self {
            ConsoleAction::Insert(char) => {
                app.console.insert(char);
                Action::None
            }
            ConsoleAction::Submit => {
                app.console.submit();
                Action::None
            }
            ConsoleAction::Backspace => {
                app.console.backspace();
                Action::None
            }
            ConsoleAction::NavigateBackwards => {
                app.console.navigate_backwards();
                Action::None
            }
            ConsoleAction::NavigateForwards => {
                app.console.navigate_forwards();
                Action::None
            }
            ConsoleAction::ShiftLeft => {
                app.console.shift_left();
                Action::None
            }
            ConsoleAction::ShiftRight => {
                app.console.shift_right();
                Action::None
            }
            ConsoleAction::ShiftHome => {
                app.console.shift_home();
                Action::None
            }
            ConsoleAction::ShiftEnd => {
                app.console.shift_end();
                Action::None
            }
        }
    }
}

impl From<ConsoleAction> for Action {
    fn from(action: ConsoleAction) -> Self {
        Action::Console(action)
    }
}
