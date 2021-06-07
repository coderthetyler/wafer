use cgmath::{Angle, Deg, InnerSpace, Matrix4, Vector3};

use crate::{input, time::Seconds};

pub trait Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32>;
    fn update_aspect(&mut self, width: f32, height: f32);
    fn update(&mut self, inputs: &input::Inputs, delta: Seconds);
}

pub struct FreeCamera {
    position: cgmath::Vector3<f32>,
    pitch: f32,
    yaw: f32,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
    sensitivity: f32,
}

impl FreeCamera {
    pub fn new(speed: f32, width: u32, height: u32) -> Self {
        Self {
            position: (0.0, 0.0, 32.0).into(),
            pitch: 0.0,
            yaw: 180.0,
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed,
            sensitivity: 1.0 / 7.0,
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
}

impl Camera for FreeCamera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::from_angle_x(Deg(self.pitch))
            * Matrix4::from_angle_y(Deg(self.yaw))
            * Matrix4::from_translation(self.position);
        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn update(&mut self, inputs: &input::Inputs, delta: Seconds) {
        let speed = self.speed * delta.as_f32();
        let (yaw_delta, pitch_delta) = inputs.mouse_delta();
        self.yaw += yaw_delta as f32 * self.sensitivity;
        self.yaw %= 360.0;
        self.pitch += pitch_delta as f32 * self.sensitivity;
        self.pitch = self.pitch.min(90.0).max(-90.0);

        let forward: Vector3<f32> = self.get_forward_vector().into();
        let forward = forward.normalize();
        let right: Vector3<f32> = self.get_right_vector().into();
        let right = right.normalize();
        let unit_up: Vector3<f32> = [0.0, -1.0, 0.0].into();

        let mut delta: Vector3<f32> = [0.0, 0.0, 0.0].into();
        if inputs.is_forward_pressed {
            delta += forward;
        }
        if inputs.is_backward_pressed {
            delta -= forward;
        }
        if inputs.is_up_pressed {
            delta += unit_up;
        }
        if inputs.is_down_pressed {
            delta -= unit_up;
        }
        if inputs.is_right_pressed {
            delta += right;
        }
        if inputs.is_left_pressed {
            delta -= right;
        }
        if delta.magnitude2() != 0.0 {
            let delta = speed * delta.normalize();
            self.position += delta;
        }
    }
}
