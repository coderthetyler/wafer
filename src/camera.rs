use cgmath::{Deg, Matrix4, Rad};

use crate::input;

pub trait Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32>;
    fn update_aspect(&mut self, width: f32, height: f32);
    fn update(&mut self, inputs: &input::Inputs);
    // TODO add a forward vector for CPU-side voxel face mesh culling
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
        }
    }
}

impl Camera for FreeCamera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::from_angle_x(Rad(self.pitch))
            * Matrix4::from_angle_y(Rad(self.yaw))
            * Matrix4::from_translation(self.position);

        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    fn update(&mut self, inputs: &input::Inputs) {
        // capture mouse
        // parse mouse move events into controller
        // update pitch and yaw with mouse movements
        // calculate forward and up vectors
        // move at speed along vectors: -forward, forward, -up, up
        // todo!()
    }
}

// TODO please make the camera better
// Stolen from https://github.com/sotrh/learn-wgpu
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
        use cgmath::InnerSpace;
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
