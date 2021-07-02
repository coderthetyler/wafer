use winit::{
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::action::{Action, ConsoleAction, SceneAction, WindowAction};

use super::EventAction;

/// Queryable input state.
/// State is driven by emitted [`SceneAction`].
#[derive(Default)]
pub struct SceneInputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub pan: Option<(f32, f32)>,
}

impl SceneInputState {
    /// Reset the input state.
    /// This should be called at the beginning of a new frame.
    /// Internal state with toggle semantics is ignored.
    pub fn reset(&mut self) {
        self.pan = None;
    }

    /// Update the input state with an emitted [`SceneAction`].
    pub fn update(&mut self, action: SceneAction) {
        match action {
            SceneAction::Forward(state) => self.forward = state,
            SceneAction::Backward(state) => self.backward = state,
            SceneAction::Left(state) => self.left = state,
            SceneAction::Right(state) => self.right = state,
            SceneAction::Up(state) => self.up = state,
            SceneAction::Down(state) => self.down = state,
            SceneAction::Pan(x, y) => {
                // We want to accumulate pan actions (more than one can occur between frames)
                if let Some(prior_pan) = self.pan {
                    self.pan = Some((prior_pan.0 + x, prior_pan.1 + y));
                } else {
                    self.pan = Some((x, y));
                }
            }
        }
    }
}

/// Interprets events as inputs to the scene.
pub struct SceneInputContext {}

impl SceneInputContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn on_active(&mut self) -> Option<Action> {
        Some(Action::Window(WindowAction::GrabCursor))
    }

    #[allow(clippy::collapsible_match, clippy::single_match)]
    pub fn interpret(&mut self, wid: &WindowId, event: &Event<()>) -> EventAction {
        match event {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    SceneAction::Pan(delta.0 as f32, delta.1 as f32).into()
                }
                _ => EventAction::Unconsumed,
            },
            Event::WindowEvent { window_id, event } if wid == window_id => match event {
                WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state,
                            virtual_keycode: Some(key_code),
                            ..
                        },
                    ..
                } => {
                    let is_pressed = *state == ElementState::Pressed;
                    match key_code {
                        VirtualKeyCode::T => ConsoleAction::Show.into(),
                        VirtualKeyCode::Space => SceneAction::Up(is_pressed).into(),
                        VirtualKeyCode::LShift => SceneAction::Down(is_pressed).into(),
                        VirtualKeyCode::W => SceneAction::Forward(is_pressed).into(),
                        VirtualKeyCode::A => SceneAction::Left(is_pressed).into(),
                        VirtualKeyCode::S => SceneAction::Backward(is_pressed).into(),
                        VirtualKeyCode::D => SceneAction::Right(is_pressed).into(),
                        _ => EventAction::Unconsumed,
                    }
                }
                _ => EventAction::Unconsumed,
            },
            _ => EventAction::Unconsumed,
        }
    }
}
