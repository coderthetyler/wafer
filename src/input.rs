use crate::entity::EntitySystem;

const MOUSE_SMOOTH_FRAMES: usize = 4;

pub struct InputComponent {}

pub struct UserInputFrame {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    mouse_deltas: [(f64, f64); MOUSE_SMOOTH_FRAMES],
}

impl UserInputFrame {
    pub fn new() -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_deltas: [(0.0, 0.0); MOUSE_SMOOTH_FRAMES],
        }
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        let mut delta = (0.0, 0.0);
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for i in 0..MOUSE_SMOOTH_FRAMES {
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
        for i in (1..MOUSE_SMOOTH_FRAMES).rev() {
            self.mouse_deltas[i] = self.mouse_deltas[i - 1];
        }
        self.mouse_deltas[0] = (0.0, 0.0);
    }
}

pub struct InputSystem {
    input: UserInputFrame,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            input: UserInputFrame::new(),
        }
    }

    pub fn update(&mut self, entities: &mut EntitySystem) {
        self.input.shift_mouse_deltas();
    }

    #[allow(clippy::collapsible_match)]
    pub fn receive_events(
        &mut self,
        src_window: &winit::window::WindowId,
        event: &winit::event::Event<()>,
    ) -> bool {
        match event {
            winit::event::Event::DeviceEvent { ref event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    self.input.inc_mouse_delta(delta);
                    true
                }
                _ => false,
            },
            winit::event::Event::WindowEvent { window_id, event } if src_window == window_id => {
                match event {
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => {
                        let is_pressed = *state == winit::event::ElementState::Pressed;
                        match keycode {
                            winit::event::VirtualKeyCode::Space => {
                                self.input.is_up_pressed = is_pressed;
                                true
                            }
                            winit::event::VirtualKeyCode::LShift => {
                                self.input.is_down_pressed = is_pressed;
                                true
                            }
                            winit::event::VirtualKeyCode::W | winit::event::VirtualKeyCode::Up => {
                                self.input.is_forward_pressed = is_pressed;
                                true
                            }
                            winit::event::VirtualKeyCode::A
                            | winit::event::VirtualKeyCode::Left => {
                                self.input.is_left_pressed = is_pressed;
                                true
                            }
                            winit::event::VirtualKeyCode::S
                            | winit::event::VirtualKeyCode::Down => {
                                self.input.is_backward_pressed = is_pressed;
                                true
                            }
                            winit::event::VirtualKeyCode::D
                            | winit::event::VirtualKeyCode::Right => {
                                self.input.is_right_pressed = is_pressed;
                                true
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
