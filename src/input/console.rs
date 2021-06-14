use winit::{
    event::{Event, VirtualKeyCode, WindowEvent::KeyboardInput},
    window::WindowId,
};
use Event::WindowEvent;

use crate::{console::Console, entity::EntitySystem, time::Seconds};

use super::ContextAction;

pub struct ConsoleInputContext {
    console: Console,
}

impl ConsoleInputContext {
    pub fn new(console: Console) -> Self {
        Self { console }
    }

    #[allow(clippy::single_match, clippy::collapsible_match)]
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> ContextAction {
        match event {
            WindowEvent { window_id, event } if windowid == window_id => match event {
                KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        return ContextAction::Pop;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        ContextAction::UnconsumedEvent
    }

    pub fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        todo!()
    }
}
