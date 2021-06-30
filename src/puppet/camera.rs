use cgmath::{Angle, Deg, InnerSpace, Vector3};
use winit::{
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::{
    action::{Action, ConsoleAction},
    geometry::{Position, Rotation},
    input::EventAction,
    time::Frame,
};

#[derive(Default)]
pub struct FreeCameraPuppet {
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    mouse_deltas: [(f64, f64); Self::MOUSE_SMOOTH_FRAMES],
}

impl FreeCameraPuppet {
    const MOUSE_SMOOTH_FRAMES: usize = 4;

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

    fn rotate_mouse_deltas(&mut self) {
        for i in (1..Self::MOUSE_SMOOTH_FRAMES).rev() {
            self.mouse_deltas[i] = self.mouse_deltas[i - 1];
        }
        self.mouse_deltas[0] = (0.0, 0.0);
    }

    #[allow(clippy::collapsible_match, clippy::single_match)]
    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        match event {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.inc_mouse_delta(delta);
                    EventAction::Consumed
                }
                _ => EventAction::Unconsumed,
            },
            Event::WindowEvent { window_id, event } if windowid == window_id => match event {
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
                        VirtualKeyCode::T => Action::Console(ConsoleAction::Show).into(),
                        VirtualKeyCode::Space => {
                            self.is_up_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        VirtualKeyCode::LShift => {
                            self.is_down_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        VirtualKeyCode::W => {
                            self.is_forward_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        VirtualKeyCode::A => {
                            self.is_left_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        VirtualKeyCode::S => {
                            self.is_backward_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        VirtualKeyCode::D => {
                            self.is_right_pressed = is_pressed;
                            EventAction::Consumed
                        }
                        _ => EventAction::Unconsumed,
                    }
                }
                _ => EventAction::Unconsumed,
            },
            _ => EventAction::Unconsumed,
        }
    }

    fn get_forward_vector(yaw: f32, pitch: f32) -> [f32; 3] {
        let yaw = Deg(-yaw);
        let pitch = Deg(-pitch);
        [
            pitch.cos() * yaw.sin(),
            -pitch.sin(),
            pitch.cos() * yaw.cos(),
        ]
    }

    fn get_right_vector(yaw: f32) -> [f32; 3] {
        let yaw = Deg(-yaw);
        [-yaw.cos(), 0.0, yaw.sin()]
    }

    pub fn update(
        &mut self,
        frame: &Frame,
        position: Option<&mut Position>,
        rotation: Option<&mut Rotation>,
    ) {
        if let (Some(pos), Some(rot)) = (position, rotation) {
            let speed = 20.0;
            let sensitivity = 0.1;

            let speed = speed * frame.delta.as_f32();
            let (yaw_delta, pitch_delta) = self.mouse_delta();

            // yaw
            rot.y += yaw_delta as f32 * sensitivity;
            rot.y %= 360.0;
            let yaw = rot.y;

            // pitch
            rot.x += pitch_delta as f32 * sensitivity;
            rot.x = rot.x.min(90.0).max(-90.0);
            let pitch = rot.x;

            let forward: Vector3<f32> = Self::get_forward_vector(yaw, pitch).into();
            let forward = forward.normalize();
            let right: Vector3<f32> = Self::get_right_vector(yaw).into();
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
                pos.x += delta.x;
                pos.y += delta.y;
                pos.z += delta.z;
            }
        }
        self.rotate_mouse_deltas();
    }
}
