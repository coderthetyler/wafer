use std::time::{SystemTime, UNIX_EPOCH};

const MOUSE_SMOOTH_FRAMES: usize = 4;

pub struct Inputs {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub last_time: u128,
    pub delta_time: f32,
    mouse_deltas: [(f64, f64); MOUSE_SMOOTH_FRAMES], // TODO use as ring buffer!
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
            mouse_deltas: [(0.0, 0.0); MOUSE_SMOOTH_FRAMES],
            last_time: now,
            delta_time: 0.0,
        }
    }

    pub fn inc_mouse_delta(&mut self, delta: &(f64, f64)) {
        self.mouse_deltas[0] = (
            self.mouse_deltas[0].0 + delta.0,
            self.mouse_deltas[0].1 + delta.1,
        );
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

    pub fn clear(&mut self) {
        for i in (1..MOUSE_SMOOTH_FRAMES).rev() {
            self.mouse_deltas[i] = self.mouse_deltas[i - 1];
        }
        self.mouse_deltas[0] = (0.0, 0.0);
    }

    pub fn now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_millis()
    }
}
