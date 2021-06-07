use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub struct Seconds(f32);

impl Seconds {
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Timestamp(u128);

impl Timestamp {
    pub fn now() -> Timestamp {
        Timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_millis(),
        )
    }

    pub fn delta(&self, relative_to: Timestamp) -> Seconds {
        Seconds((self.0 - relative_to.0) as f32 / 1000.0)
    }
}
