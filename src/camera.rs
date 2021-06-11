use cgmath::{Angle, Deg, InnerSpace, Matrix4, Vector3};

use crate::{input::UserInputFrame, time::Seconds};

pub enum CameraMode {
    Free,
}

pub struct Camera {
    action: CameraMode,
    position: cgmath::Vector3<f32>,
    pitch: f32,
    yaw: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new(speed: f32, sensitivity: f32, action: CameraMode) -> Self {
        match action {
            CameraMode::Free => Camera::new_free_camera(speed, sensitivity),
        }
    }

    pub fn new_free_camera(speed: f32, sensitivity: f32) -> Self {
        Self {
            action: CameraMode::Free,
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

    pub fn build_view_projection_matrix(&self, aspect_ratio: AspectRatio) -> Matrix4<f32> {
        let view = Matrix4::from_angle_x(Deg(self.pitch))
            * Matrix4::from_angle_y(Deg(self.yaw))
            * Matrix4::from_translation(self.position);
        let proj = cgmath::perspective(Deg(self.fovy), aspect_ratio.into(), self.znear, self.zfar);
        proj * view
    }

    pub fn update(&mut self, input: &UserInputFrame, delta: Seconds) {
        match self.action {
            CameraMode::Free => self.update_free(input, delta),
        }
    }

    fn get_forward_vector(&self) -> [f32; 3] {
        let yaw = Deg(-self.yaw);
        let pitch = Deg(-self.pitch);
        [
            pitch.cos() * yaw.sin(),
            -pitch.sin(),
            pitch.cos() * yaw.cos(),
        ]
    }

    fn get_right_vector(&self) -> [f32; 3] {
        let yaw = Deg(-self.yaw);
        [-yaw.cos(), 0.0, yaw.sin()]
    }

    /// Update as a free camera
    fn update_free(&mut self, input: &UserInputFrame, delta: Seconds) {
        let speed = self.speed * delta.as_f32();
        let (yaw_delta, pitch_delta) = input.mouse_delta();
        self.yaw += yaw_delta as f32 * self.sensitivity;
        self.yaw %= 360.0;
        self.pitch += pitch_delta as f32 * self.sensitivity;
        self.pitch = self.pitch.min(90.0).max(-90.0);

        let forward: Vector3<f32> = self.get_forward_vector().into();
        let forward = forward.normalize();
        let right: Vector3<f32> = self.get_right_vector().into();
        let right = right.normalize();
        let up: Vector3<f32> = forward.cross(right).normalize();

        let mut delta: Vector3<f32> = [0.0, 0.0, 0.0].into();
        if input.is_forward_pressed {
            delta += forward;
        }
        if input.is_backward_pressed {
            delta -= forward;
        }
        if input.is_up_pressed {
            delta += up;
        }
        if input.is_down_pressed {
            delta -= up;
        }
        if input.is_right_pressed {
            delta += right;
        }
        if input.is_left_pressed {
            delta -= right;
        }
        if delta.magnitude2() != 0.0 {
            let delta = speed * delta.normalize();
            self.position += delta;
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
