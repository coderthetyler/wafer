use cgmath::{Angle, Deg, InnerSpace, Matrix4, Vector3};

use crate::input;

pub trait Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32>;
    fn update_aspect(&mut self, width: f32, height: f32);
    fn update(&mut self, inputs: &input::Inputs);
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
            sensitivity: 7.0,
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

    fn update(&mut self, inputs: &input::Inputs) {
        let speed = self.speed * inputs.delta_time;
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

// Mostly stolen from https://github.com/sotrh/learn-wgpu
pub struct TargetCamera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
}

impl TargetCamera {
    pub fn new(speed: f32, target: [f32; 3], distance: f32, width: u32, height: u32) -> Self {
        Self {
            eye: (target[0], target[1], target[2] + distance).into(),
            target: target.into(),
            up: cgmath::Vector3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 500.0,
            speed,
        }
    }
}

impl Camera for TargetCamera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn update(&mut self, inputs: &input::Inputs) {
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if inputs.is_forward_pressed && forward_mag > self.speed {
            self.eye += forward_norm * self.speed;
        }
        if inputs.is_backward_pressed {
            self.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(self.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = self.target - self.eye;
        let forward_mag = forward.magnitude();

        if inputs.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            self.eye = self.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if inputs.is_left_pressed {
            self.eye = self.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
