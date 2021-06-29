pub struct Camera {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
        }
    }
}

pub struct AspectRatio(f32);

impl From<(f32, f32)> for AspectRatio {
    fn from((width, height): (f32, f32)) -> Self {
        AspectRatio(width / height)
    }
}

impl From<f32> for AspectRatio {
    fn from(aspect_ratio: f32) -> Self {
        AspectRatio(aspect_ratio)
    }
}

impl From<AspectRatio> for f32 {
    fn from(aspect_ratio: AspectRatio) -> Self {
        aspect_ratio.0
    }
}
