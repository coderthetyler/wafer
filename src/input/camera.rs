use cgmath::{InnerSpace, Vector3};
use winit::{
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::{
    action::{Action, InputSystemAction, WindowAction},
    entity::{Entity, EntitySystem},
    time::Seconds,
};

use super::{ConsoleInputContext, EventAction};

pub struct CameraInputContext {
    camera: Entity,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    mouse_deltas: [(f64, f64); CameraInputContext::MOUSE_SMOOTH_FRAMES],
}

impl CameraInputContext {
    const MOUSE_SMOOTH_FRAMES: usize = 4;

    pub fn new(camera: Entity) -> Self {
        Self {
            camera,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_deltas: [(0.0, 0.0); Self::MOUSE_SMOOTH_FRAMES],
        }
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        let mut delta = (0.0, 0.0);
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for i in 0..Self::MOUSE_SMOOTH_FRAMES {
            delta.0 += self.mouse_deltas[i].0 * weight;
            delta.1 += self.mouse_deltas[i].1 * weight;
            total_weight += weight;
            weight /= 2.0;
        }
        (delta.0 / total_weight, delta.1 / total_weight)
    }

    fn inc_mouse_delta(&mut self, delta: &(f64, f64)) {
        self.mouse_deltas[0] = (
            self.mouse_deltas[0].0 + delta.0,
            self.mouse_deltas[0].1 + delta.1,
        );
    }

    fn shift_mouse_deltas(&mut self) {
        for i in (1..Self::MOUSE_SMOOTH_FRAMES).rev() {
            self.mouse_deltas[i] = self.mouse_deltas[i - 1];
        }
        self.mouse_deltas[0] = (0.0, 0.0);
    }

    pub(super) fn update(&mut self, entities: &mut EntitySystem, delta: Seconds) {
        if let Some(camera) = entities.cameras.get_mut(self.camera) {
            let speed = camera.speed * delta.as_f32();
            let (yaw_delta, pitch_delta) = self.mouse_delta();
            camera.yaw += yaw_delta as f32 * camera.sensitivity;
            camera.yaw %= 360.0;
            camera.pitch += pitch_delta as f32 * camera.sensitivity;
            camera.pitch = camera.pitch.min(90.0).max(-90.0);

            let forward: Vector3<f32> = camera.get_forward_vector().into();
            let forward = forward.normalize();
            let right: Vector3<f32> = camera.get_right_vector().into();
            let right = right.normalize();
            let up: Vector3<f32> = forward.cross(right).normalize();

            let mut delta: Vector3<f32> = [0.0, 0.0, 0.0].into();
            if self.is_forward_pressed {
                delta += forward;
            }
            if self.is_backward_pressed {
                delta -= forward;
            }
            if self.is_up_pressed {
                delta += up;
            }
            if self.is_down_pressed {
                delta -= up;
            }
            if self.is_right_pressed {
                delta += right;
            }
            if self.is_left_pressed {
                delta -= right;
            }
            if delta.magnitude2() != 0.0 {
                let delta = speed * delta.normalize();
                camera.position += delta;
            }
        }
        self.shift_mouse_deltas();
    }

    pub(super) fn on_active(&mut self) -> Action {
        Action::Window(WindowAction::GrabCursor)
    }

    #[allow(clippy::collapsible_match, clippy::single_match)]
    pub(super) fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        match event {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.inc_mouse_delta(delta);
                    return EventAction::Consumed;
                }
                _ => {}
            },
            Event::WindowEvent { window_id, event } if windowid == window_id => match event {
                WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    let is_pressed = *state == ElementState::Pressed;
                    match keycode {
                        VirtualKeyCode::T => {
                            return Action::InputSystem(InputSystemAction::PushContext(
                                ConsoleInputContext::new().into(),
                            ))
                            .into();
                        }
                        VirtualKeyCode::Space => {
                            self.is_up_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        VirtualKeyCode::LShift => {
                            self.is_down_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        VirtualKeyCode::W => {
                            self.is_forward_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        VirtualKeyCode::A => {
                            self.is_left_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        VirtualKeyCode::S => {
                            self.is_backward_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        VirtualKeyCode::D => {
                            self.is_right_pressed = is_pressed;
                            return EventAction::Consumed;
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
        EventAction::Unconsumed
    }
}
