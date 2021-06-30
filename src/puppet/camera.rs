use cgmath::{Angle, Deg, InnerSpace, Vector3};

use crate::{
    entity::{Entity, EntityComponents, EntityDelta},
    frame::Frame,
    geometry::Rotation,
    payload::Payload,
};

#[derive(Default)]
pub struct FreeCameraPuppet {
    mouse_deltas: [(f32, f32); Self::MOUSE_SMOOTH_FRAMES],
}

impl FreeCameraPuppet {
    const MOUSE_SMOOTH_FRAMES: usize = 4;

    pub fn calculate_smooth_pan(&self) -> (f32, f32) {
        let mut delta = (0.0, 0.0);
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for i in 0..Self::MOUSE_SMOOTH_FRAMES {
            delta.0 += self.mouse_deltas[i].0 * weight;
            delta.1 += self.mouse_deltas[i].1 * weight;
            total_weight += weight;
            weight /= 2.0;
        }
        (delta.0 / total_weight, delta.1 / total_weight)
    }

    pub fn pre_update(&mut self, frame: &Frame) {
        for i in (1..Self::MOUSE_SMOOTH_FRAMES).rev() {
            self.mouse_deltas[i] = self.mouse_deltas[i - 1];
        }
        if let Some(pan) = frame.input.pan {
            self.mouse_deltas[0] = pan;
        } else {
            self.mouse_deltas[0] = (0.0, 0.0);
        }
    }

    pub fn gen_deltas(
        &self,
        frame: &Frame,
        entity: Entity,
        components: &EntityComponents,
    ) -> Payload<EntityDelta> {
        let mut payload: Payload<EntityDelta> = Payload::new();

        let mut forward: Vector3<f32> = Vector3::unit_z();
        let mut right: Vector3<f32> = -Vector3::unit_x();

        // rotation
        if let Some(rot) = components.rotation.get(entity) {
            let sensitivity = 0.1;

            let (yaw_delta, pitch_delta) = self.calculate_smooth_pan();
            let mut yaw = rot.yaw;
            yaw += yaw_delta as f32 * sensitivity;
            yaw %= 360.0;

            let mut pitch = rot.pitch;
            pitch += pitch_delta as f32 * sensitivity;
            pitch = pitch.min(90.0).max(-90.0);

            payload +=
                EntityDelta::SetRotation(Rotation::default().with_yaw(yaw).with_pitch(pitch));

            fn get_forward_vector(yaw: f32, pitch: f32) -> [f32; 3] {
                let yaw = Deg(-yaw);
                let pitch = Deg(-pitch);
                [
                    pitch.cos() * yaw.sin(),
                    -pitch.sin(),
                    pitch.cos() * yaw.cos(),
                ]
            }

            fn get_right_vector(yaw: f32) -> [f32; 3] {
                let yaw = Deg(-yaw);
                [-yaw.cos(), 0.0, yaw.sin()]
            }

            forward = get_forward_vector(yaw, pitch).into();
            right = get_right_vector(yaw).into();
            forward = forward.normalize();
            right = right.normalize();
        }

        // position
        if components.position.get(entity).is_some() {
            let speed = 20.0;

            let up: Vector3<f32> = forward.cross(right).normalize();
            let mut delta: Vector3<f32> = [0.0, 0.0, 0.0].into();
            if frame.input.forward {
                delta += forward;
            }
            if frame.input.backward {
                delta -= forward;
            }
            if frame.input.up {
                delta += up;
            }
            if frame.input.down {
                delta -= up;
            }
            if frame.input.right {
                delta += right;
            }
            if frame.input.left {
                delta -= right;
            }
            if delta.magnitude2() != 0.0 {
                let speed = speed * frame.delta.as_f32();
                let delta = speed * delta.normalize();
                payload += EntityDelta::Translate(delta.x, delta.y, delta.z);
            }
        }
        payload
    }
}
