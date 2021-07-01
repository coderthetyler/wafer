use crate::{
    input::scene::SceneInputState,
    types::{Seconds, Timestamp},
};

/// Queryable information about a single frame.
pub struct Frame {
    /// Number of frames per second, also known as the framerate.
    /// This is calculated using a smoothing function.
    /// As such, it is **not** equal to `1.0 / delta`.
    pub framerate: f32,
    /// Time since last frame.
    pub delta: Seconds,
    /// Timestamp of beginning of this frame.
    pub instant: Timestamp,
    /// Queryable input state.
    pub input: SceneInputState,
    /// Circular buffer of prior frame deltas used to compute a smoothed framerate.
    prior_deltas: [Seconds; Self::SMOOTH_COUNT],
    /// Start index into `framerate_buffer`
    buffer_head: usize,
}

impl Frame {
    const SMOOTH_COUNT: usize = 8;

    pub fn new() -> Self {
        Self {
            framerate: 0.0,
            delta: Seconds(0.0),
            instant: Timestamp::now(),
            input: SceneInputState::default(),
            prior_deltas: [Seconds(0.0); Frame::SMOOTH_COUNT],
            buffer_head: 0,
        }
    }

    /// Start a new frame.
    /// - Set a new `instant`.
    /// - Update `delta` time with new `instant`.
    /// - Cycle framerate smoothing buffer.
    /// - Calculate the `framerate`.
    pub fn start(&mut self) {
        let prior_instant = self.instant;
        self.instant = Timestamp::now();
        self.delta = self.instant.delta(prior_instant);

        self.buffer_head = (self.buffer_head + 1) % Frame::SMOOTH_COUNT;
        self.prior_deltas[self.buffer_head] = self.delta;

        let mut weighted_delta = 0.0;
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for i in 0..Frame::SMOOTH_COUNT {
            let index = (Frame::SMOOTH_COUNT + self.buffer_head - i) % Frame::SMOOTH_COUNT;
            weighted_delta += self.prior_deltas[index].0 * weight;
            total_weight += weight;
            weight /= 2.0;
        }
        weighted_delta /= total_weight;
        self.framerate = 1.0 / weighted_delta;
    }

    /// Clean up after a frame completes.
    pub fn end(&mut self) {
        self.input.reset();
    }
}
