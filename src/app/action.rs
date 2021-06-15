use ascii::AsciiChar;

use crate::input::InputContext;

use super::Application;

pub enum Action {
    /// Indicates the absence of an action, semantically similar to [`Option::None`].
    /// This is intended to indicate that an action producer was unable to produce one.
    None,
    /// Action that performs no operation.
    NoOp,
    /// An action scoped to the input system.
    InputSystem(InputSystemAction),
    /// An action scoped to the console.
    Console(ConsoleAction),
}

pub enum InputSystemAction {
    /// Pop the current input context.
    PopContext,
    /// Push a new input context & make it active.
    PushContext(InputContext),
}

impl InputSystemAction {
    pub fn perform(self, app: &mut Application) {
        match self {
            InputSystemAction::PopContext => {
                app.input_system.pop_context();
            }
            InputSystemAction::PushContext(context) => {
                app.input_system.push_context(context);
            }
        }
    }
}

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
    pub fn perform(self, app: &mut Application) {
        match self {
            ConsoleAction::Insert(char) => {
                app.console.insert(char);
            }
            ConsoleAction::Submit => {
                app.console.submit();
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
