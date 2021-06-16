use ascii::AsciiChar;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::{
    action::{Action, ConsoleAction, InputSystemAction, WindowAction},
    entity::EntitySystem,
    time::Seconds,
};

use super::EventAction;

pub struct ConsoleInputContext {}

impl ConsoleInputContext {
    pub fn new() -> Self {
        Self {}
    }

    pub(super) fn on_active(&mut self) -> Action {
        Action::Window(WindowAction::UngrabCursor)
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub(super) fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        fn receive_virtual_keycode(code: VirtualKeyCode) -> EventAction {
            match code {
                VirtualKeyCode::Escape => Action::InputSystem(InputSystemAction::PopContext).into(),
                VirtualKeyCode::Return => Action::Console(ConsoleAction::Submit).into(),
                VirtualKeyCode::Delete => Action::Console(ConsoleAction::Backspace).into(),
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
                        return Action::Console(ConsoleAction::Insert(ascii_char)).into();
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(code) = input.virtual_keycode {
                        return receive_virtual_keycode(code);
                    }
                }
                _ => {}
            },
            _ => {}
        }
        EventAction::Unconsumed
    }

    pub(super) fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {}
}
