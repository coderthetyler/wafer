use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    camera::Camera,
    console::Console,
    draw::DrawSystem,
    entity::EntitySystem,
    input::{CameraInputContext, EventAction, InputSystem},
    time::Timestamp,
};

pub struct Application {
    pub window: Window,
    pub console: Console,
    pub draw_system: DrawSystem,
    pub entity_system: EntitySystem,
    pub input_system: InputSystem,
    fallback_camera: Camera,
    last_frame: Timestamp,
}

impl Application {
    pub async fn new(window: Window) -> Self {
        let draw_system = DrawSystem::new(&window).await;
        let mut app = Application {
            window,
            console: Console::new(),
            draw_system,
            entity_system: EntitySystem::new(),
            input_system: InputSystem::new(),
            fallback_camera: Camera::new(10.0, 0.1),
            last_frame: Timestamp::now(),
        };

        let player_camera = app.entity_system.create();
        app.entity_system
            .cameras
            .set(player_camera, Camera::new(20.0, 0.1));
        app.entity_system.selected_camera = player_camera;

        app.input_system
            .push_context(CameraInputContext::new(player_camera).into())
            .perform(&mut app);

        app
    }

    pub fn receive_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        let action = self.input_system.receive_event(&self.window.id(), event);
        match action {
            EventAction::Unconsumed => {
                self.process_app_events(event, control_flow);
            }
            EventAction::Consumed => {
                // Nothing to do if event was consumed without producing an action
            }
            EventAction::React(action) => {
                action.perform(self);
            }
        }
    }

    /// Process all top-level events that drive basic application behavior.
    /// This includes window resizing and redraw requests, for example.
    fn process_app_events(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::MainEventsCleared => self.window.request_redraw(),
            Event::RedrawRequested(window_id) if *window_id == self.window.id() => self.redraw(),
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
                        if let ElementState::Pressed = input.state {
                            *control_flow = ControlFlow::Exit
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn redraw(&mut self) {
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
}
