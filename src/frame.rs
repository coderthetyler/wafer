use crate::{
    input::SceneInputState,
    types::{CircularVec, Falloff, Seconds, Timestamp},
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
    deltas: CircularVec<Seconds>,
    /// Weight falloff function used to smooth framerate calculation.
    falloff: Falloff,
}

impl Frame {
    pub fn new(len: usize, falloff: Falloff) -> Self {
        Self {
            framerate: 0.0,
            delta: Seconds(0.0),
            instant: Timestamp::now(),
            input: SceneInputState::default(),
            deltas: CircularVec::with_len(len),
            falloff,
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

        self.deltas.advance();
        self.deltas.replace(self.delta);

        let mut weighted_delta = 0.0;
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for delta in self.deltas.iter_rev() {
            weighted_delta += delta.as_f32() * weight;
            total_weight += weight;
            weight = self.falloff.apply(weight);
        }
        weighted_delta /= total_weight;
        self.framerate = 1.0 / weighted_delta;
    }

    /// Clean up after a frame completes.
    pub fn end(&mut self) {
        self.input.reset();
    }
}
