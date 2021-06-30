use winit::{event::Event, window::WindowId};

use crate::{
    action::{Action, WindowAction},
    entity::{EntityComponents, EntityPool},
    time::Frame,
};

use super::EventAction;

pub struct PuppetInputContext {}

impl PuppetInputContext {
    pub fn new() -> Self {
        Self {}
    }
}

impl PuppetInputContext {
    pub fn on_active(&mut self) -> Option<Action> {
        Some(Action::Window(WindowAction::GrabCursor))
    }

    pub fn receive_event(
        &mut self,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
        windowid: &WindowId,
        event: &Event<()>,
    ) -> EventAction {
        for entity in entities.iter() {
            if let Some(controller) = components.puppet.get_mut(entity) {
                let result = controller.receive_event(windowid, event);
                if let EventAction::Unconsumed = result {
                } else {
                    return result;
                }
            }
        }
        EventAction::Unconsumed
    }

    pub fn update(
        &self,
        frame: &Frame,
        entities: &mut EntityPool,
        components: &mut EntityComponents,
    ) {
        for entity in entities.iter() {
            if let Some(controller) = components.puppet.get_mut(entity) {
                let position = components.position.get_mut(entity);
                let rotation = components.rotation.get_mut(entity);
                controller.update(frame, position, rotation);
            }
        }
    }
}
