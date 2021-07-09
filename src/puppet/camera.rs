use core::f32;

use cgmath::{Angle, Deg, InnerSpace, Vector3};

use crate::{
    entity::{Ecs, Entity, EntityDelta},
    frame::Frame,
    types::{CircularVec, Falloff, Payload, Rotation},
};

use super::Puppet;

pub struct FreeCameraPuppet {
    history: CircularVec<(f32, f32)>,
    falloff: Falloff,
}

impl From<FreeCameraPuppet> for Puppet {
    fn from(puppet: FreeCameraPuppet) -> Self {
        Puppet::FreeCamera(puppet)
    }
}

impl FreeCameraPuppet {
    /// Create a new free camera with inter-frame pan smoothing.
    /// The number of frames cached is `len`.
    /// The weight of prior frames is geometrically determined by the `falloff`.
    pub fn new(len: usize, falloff: Falloff) -> Self {
        Self {
            history: CircularVec::with_len(len),
            falloff,
        }
    }

    fn calculate_smooth_pan(&self) -> (f32, f32) {
        let mut delta = (0.0, 0.0);
        let mut weight = 1.0;
        let mut total_weight = 0.0;
        for pan in self.history.iter_rev() {
            delta.0 += pan.0 * weight;
            delta.1 += pan.1 * weight;
            total_weight += weight;
            weight = self.falloff.apply(weight);
        }
        (delta.0 / total_weight, delta.1 / total_weight)
    }

    pub fn pre_update(&mut self, frame: &Frame) {
        self.history.advance();
        if let Some(pan) = frame.input.pan {
            self.history.replace(pan);
        } else {
            self.history.replace((0.0, 0.0));
        }
    }

    pub fn gen_deltas(&self, frame: &Frame, entity: Entity, ecs: &Ecs) -> Payload<EntityDelta> {
        let mut payload: Payload<EntityDelta> = Payload::new();

        let mut forward: Vector3<f32> = Vector3::unit_z();
        let mut right: Vector3<f32> = -Vector3::unit_x();

        // rotation
        if let Some(rot) = ecs.comps.rotation.get(entity) {
            let sensitivity = 0.1;

            let (yaw_delta, pitch_delta) = self.calculate_smooth_pan();
            let mut yaw = rot.yaw;
            yaw += yaw_delta as f32 * sensitivity;
            yaw %= 360.0;

            let mut pitch = rot.pitch;
            pitch += pitch_delta as f32 * sensitivity;
            pitch = pitch.min(90.0).max(-90.0);

            payload += EntityDelta::SetRotation(Rotation {
                pitch,
                yaw,
                roll: 0.0,
            });

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
        if ecs.comps.position.get(entity).is_some() {
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
