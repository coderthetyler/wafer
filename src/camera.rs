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
