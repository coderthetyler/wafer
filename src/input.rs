use std::time::{SystemTime, UNIX_EPOCH};

pub struct Inputs {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub mouse_delta: (f64, f64),
    pub last_time: u128,
    pub delta_time: f32,
}

impl Inputs {
    pub fn new(now: u128) -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            mouse_delta: (0.0, 0.0),
            last_time: now,
            delta_time: 0.0,
        }
    }

    pub fn clear(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_millis()
    }
}
