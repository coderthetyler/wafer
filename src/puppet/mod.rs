use winit::{event::Event, window::WindowId};

use crate::{
    geometry::{Position, Rotation},
    input::EventAction,
    time::Frame,
};

use self::camera::FreeCameraPuppet;

pub mod camera;

/// A puppet describes the high-level faculties for piloting an entity.
/// It does not provide any "mind" that calls those faculties.
/// Variants can include a free camera controller or an in-world controller constrained by physics.
pub enum Puppet {
    FreeCamera(FreeCameraPuppet),
}

impl Puppet {
    pub fn update(
        &mut self,
        frame: &Frame,
        position: Option<&mut Position>,
        rotation: Option<&mut Rotation>,
    ) {
        match self {
            Puppet::FreeCamera(controller) => controller.update(frame, position, rotation),
        }
    }

    pub fn receive_event(&mut self, windowid: &WindowId, event: &Event<()>) -> EventAction {
        match self {
            Puppet::FreeCamera(controller) => controller.receive_event(windowid, event),
        }
    }
}
