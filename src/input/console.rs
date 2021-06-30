use ascii::AsciiChar;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::{
    action::{Action, ConsoleAction, WindowAction},
    entity::{EntityComponents, EntityPool},
    time::Frame,
};

use super::EventAction;

pub struct ConsoleInputContext {}

impl ConsoleInputContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConsoleInputContext {
    pub fn on_active(&mut self) -> Option<Action> {
        Some(Action::Window(WindowAction::UngrabCursor))
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub fn receive_event(
        &mut self,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
        windowid: &WindowId,
        event: &Event<()>,
    ) -> EventAction {
        fn receive_virtual_keycode(code: VirtualKeyCode) -> EventAction {
            match code {
                VirtualKeyCode::Escape => Action::Console(ConsoleAction::Hide).into(),
                VirtualKeyCode::Return => Action::Console(ConsoleAction::Submit).into(),
                VirtualKeyCode::Back => Action::Console(ConsoleAction::Backspace).into(),
                VirtualKeyCode::Up => Action::Console(ConsoleAction::NavigateBackwards).into(),
                VirtualKeyCode::Down => Action::Console(ConsoleAction::NavigateForwards).into(),
                VirtualKeyCode::Left => Action::Console(ConsoleAction::ShiftLeft).into(),
                VirtualKeyCode::Right => Action::Console(ConsoleAction::ShiftRight).into(),
                VirtualKeyCode::Home => Action::Console(ConsoleAction::ShiftHome).into(),
                VirtualKeyCode::End => Action::Console(ConsoleAction::ShiftEnd).into(),
                _ => EventAction::Unconsumed,
            }
        }
        match event {
            Event::WindowEvent { window_id, event } if windowid == window_id => match event {
                WindowEvent::ReceivedCharacter(received_char) => {
                    let ascii_char = AsciiChar::from_ascii(*received_char);
                    if let Ok(ascii_char) = ascii_char {
                        if ascii_char.is_ascii_printable() {
                            return Action::Console(ConsoleAction::Insert(ascii_char)).into();
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

    pub fn update(
        &self,
        frame: &Frame,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
    ) {
        // TODO do input contexts really need this method?
    }
}
