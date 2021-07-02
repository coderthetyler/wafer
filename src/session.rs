use crate::{
    action::{Action, ConfigAction},
    types::Console,
};

/// Wraps a [`Console`] and supports multi-command processes.
pub struct ConsoleSession {
    is_showing: bool,
    pub console: Console,
}

impl ConsoleSession {
    pub fn new() -> Self {
        ConsoleSession {
            is_showing: false,
            console: Console::new(),
        }
    }

    pub fn submit(&mut self) -> Option<Action> {
        if let Some(txt) = self.console.submit() {
            if txt == "exit" {
                return Some(Action::Config(ConfigAction::RequestClose));
            } else if txt == "wires" {
                return Some(Action::Config(ConfigAction::ToglePaintColliderVolumes));
            }
        }
        None
    }

    pub fn show(&mut self) {
        self.is_showing = true;
    }

    pub fn hide(&mut self) {
        self.is_showing = false;
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }
}
