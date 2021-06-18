use cgmath::{Angle, Deg, Matrix4};

pub struct Camera {
    pub position: cgmath::Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            position: (0.0, 0.0, 32.0).into(),
            pitch: 0.0,
            yaw: 180.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed,
            sensitivity,
        }
    }

    pub fn get_forward_vector(&self) -> [f32; 3] {
        let yaw = Deg(-self.yaw);
        let pitch = Deg(-self.pitch);
        [
            pitch.cos() * yaw.sin(),
            -pitch.sin(),
            pitch.cos() * yaw.cos(),
        ]
    }

    pub fn get_right_vector(&self) -> [f32; 3] {
        let yaw = Deg(-self.yaw);
        [-yaw.cos(), 0.0, yaw.sin()]
    }

    pub fn build_view_projection_matrix(&self, aspect_ratio: AspectRatio) -> Matrix4<f32> {
        let view = Matrix4::from_angle_x(Deg(self.pitch))
            * Matrix4::from_angle_y(Deg(self.yaw))
            * Matrix4::from_translation(self.position);
        let proj = cgmath::perspective(Deg(self.fovy), aspect_ratio.into(), self.znear, self.zfar);
        proj * view
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
