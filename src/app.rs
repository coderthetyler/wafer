use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    camera::Camera,
    draw::DrawSystem,
    entity::EntitySystem,
    input::{CameraInputContext, InputSystem},
    time::Timestamp,
};

pub struct Application {
    window: Window,
    draw_system: DrawSystem,
    entity_system: EntitySystem,
    input_system: InputSystem,
    fallback_camera: Camera,
    last_frame: Timestamp,
}

impl Application {
    pub async fn new(window: Window) -> Self {
        let draw_system = DrawSystem::new(&window).await;

        let mut entity_system = EntitySystem::new();
        let player_camera = entity_system.create();
        entity_system
            .cameras
            .set(player_camera, Camera::new(20.0, 0.1));
        entity_system.selected_camera = player_camera;

        let mut input_system = InputSystem::new();
        input_system.push_context(CameraInputContext::new(player_camera).into());

        Application {
            window,
            draw_system,
            entity_system,
            input_system,
            fallback_camera: Camera::new(10.0, 0.1),
            last_frame: Timestamp::now(),
        }
    }

    pub fn receive_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        if self.input_system.receive_event(&self.window.id(), event) {
            return;
        }
        self.process_app_events(event, control_flow);
    }

    /// Process all top-level events that drive basic application behavior.
    /// This includes window resizing and redraw requests, for example.
    fn process_app_events(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::MainEventsCleared => self.window.request_redraw(),
            Event::RedrawRequested(window_id) if *window_id == self.window.id() => {
                let now = Timestamp::now();
                let delta_time = now.delta(self.last_frame);
                self.last_frame = now;
                self.input_system
                    .update(&mut self.entity_system, delta_time);
                let camera = self
                    .entity_system
                    .get_selected_camera()
                    .unwrap_or(&self.fallback_camera);
                self.draw_system.redraw(camera)
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if self.window.id() == *window_id => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.draw_system.resize_surface(new_inner_size)
                }
                WindowEvent::Resized(ref new_size) => self.draw_system.resize_surface(new_size),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
