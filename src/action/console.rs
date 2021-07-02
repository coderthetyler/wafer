use ascii::AsciiChar;

use crate::{app::Application, input::ConsoleInputContext};

use super::{Action, ActionType, ConfigAction};

pub enum ConsoleAction {
    /// Show the console.
    Show,
    /// Hide the console.
    Hide,
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

impl ActionType for ConsoleAction {
    fn perform(self, app: &mut Application) {
        match self {
            ConsoleAction::Show => {
                app.console.show();
                let context = ConsoleInputContext::new();
                if let Some(action) = app.interpreter.push_context(context) {
                    action.perform(app);
                }
            }
            ConsoleAction::Hide => {
                app.console.hide();
                if let Some(action) = app.interpreter.pop_context() {
                    action.perform(app);
                }
            }
            ConsoleAction::Insert(char) => {
                app.console.insert(char);
            }
            ConsoleAction::Submit => {
                if let Some(txt) = app.console.submit() {
                    if let Some(action) = parse_command(txt) {
                        action.perform(app);
                    }
                }
            }
            ConsoleAction::Backspace => {
                app.console.backspace();
            }
            ConsoleAction::NavigateBackwards => {
                app.console.navigate_backwards();
            }
            ConsoleAction::NavigateForwards => {
                app.console.navigate_forwards();
            }
            ConsoleAction::ShiftLeft => {
                app.console.shift_left();
            }
            ConsoleAction::ShiftRight => {
                app.console.shift_right();
            }
            ConsoleAction::ShiftHome => {
                app.console.shift_home();
            }
            ConsoleAction::ShiftEnd => {
                app.console.shift_end();
            }
        }
    }
}

impl From<ConsoleAction> for Action {
    fn from(action: ConsoleAction) -> Self {
        Action::Console(action)
    }
}

pub fn parse_command(txt: String) -> Option<Action> {
    if txt == "exit" {
        Some(Action::Config(ConfigAction::RequestClose))
    } else if txt == "wires" {
        Some(Action::Config(ConfigAction::ToglePaintColliderVolumes))
    } else {
        None
    }
}
