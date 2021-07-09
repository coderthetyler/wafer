use ascii::AsciiChar;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::action::{Action, ConsoleAction, WindowAction};

use super::EventAction;

/// Interprets events as inputs to the console.
pub struct ConsoleInputContext {
    /// winit fires off a ReceivedCharacter & a KeyboardInput event for each key press.
    /// However, it recommends not using the latter for typing interfaces, which is what this is.
    /// As such, this flag is set to `true` when the console is opened & set to false on the first event.
    /// This allows the first event to be ignored if it is a ReceivedCharacter.
    did_just_open: bool,
}

impl ConsoleInputContext {
    pub fn new() -> Self {
        Self {
            did_just_open: false,
        }
    }
}

impl ConsoleInputContext {
    pub fn on_active(&mut self) -> Option<Action> {
        self.did_just_open = true;
        Some(WindowAction::UngrabCursor.into())
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub fn interpret(&mut self, wid: &WindowId, event: &Event<()>) -> EventAction {
        let did_just_open = self.did_just_open;
        self.did_just_open = false;
        fn receive_virtual_keycode(code: VirtualKeyCode) -> EventAction {
            match code {
                VirtualKeyCode::Escape => ConsoleAction::Hide.into(),
                VirtualKeyCode::Return => ConsoleAction::Submit.into(),
                VirtualKeyCode::Back => ConsoleAction::Backspace.into(),
                VirtualKeyCode::Up => ConsoleAction::NavigateBackwards.into(),
                VirtualKeyCode::Down => ConsoleAction::NavigateForwards.into(),
                VirtualKeyCode::Left => ConsoleAction::ShiftLeft.into(),
                VirtualKeyCode::Right => ConsoleAction::ShiftRight.into(),
                VirtualKeyCode::Home => ConsoleAction::ShiftHome.into(),
                VirtualKeyCode::End => ConsoleAction::ShiftEnd.into(),
                _ => EventAction::Unconsumed,
            }
        }
        match event {
            Event::WindowEvent { window_id, event } if wid == window_id => match event {
                WindowEvent::ReceivedCharacter(received_char) => {
                    if !did_just_open {
                        let ascii_char = AsciiChar::from_ascii(*received_char);
                        if let Ok(ascii_char) = ascii_char {
                            if ascii_char.is_ascii_printable() {
                                return Action::Console(ConsoleAction::Insert(ascii_char)).into();
                            }
                        }
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let (Some(code), ElementState::Pressed) =
                        (input.virtual_keycode, input.state)
                    {
                        return receive_virtual_keycode(code);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        EventAction::Unconsumed
    }
}
