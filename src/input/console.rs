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

pub struct ConsoleInputContext {}

impl ConsoleInputContext {
    pub fn new() -> Self {
        Self {}
    }

    pub(super) fn on_active(&mut self) -> Action {
        Action::Window(WindowAction::UngrabCursor)
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub(super) fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> Action {
        match event {
            Event::WindowEvent { window_id, event } if windowid == window_id => match event {
                WindowEvent::ReceivedCharacter(received_char) => {
                    let ascii_char = AsciiChar::from_ascii(*received_char);
                    if let Ok(ascii_char) = ascii_char {
                        return Action::Console(ConsoleAction::Insert(ascii_char));
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(code) = input.virtual_keycode {
                        match code {
                            VirtualKeyCode::Escape => {
                                return Action::InputSystem(InputSystemAction::PopContext);
                            }
                            VirtualKeyCode::Return => {
                                return Action::Console(ConsoleAction::Submit);
                            }
                            VirtualKeyCode::Delete => {
                                return Action::Console(ConsoleAction::Backspace);
                            }
                            VirtualKeyCode::Up => {
                                return Action::Console(ConsoleAction::NavigateBackwards);
                            }
                            VirtualKeyCode::Down => {
                                return Action::Console(ConsoleAction::NavigateForwards);
                            }
                            VirtualKeyCode::Left => {
                                return Action::Console(ConsoleAction::ShiftLeft);
                            }
                            VirtualKeyCode::Right => {
                                return Action::Console(ConsoleAction::ShiftRight);
                            }
                            VirtualKeyCode::Home => {
                                return Action::Console(ConsoleAction::ShiftHome);
                            }
                            VirtualKeyCode::End => {
                                return Action::Console(ConsoleAction::ShiftEnd);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Action::None
    }

    pub(super) fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {}
}
