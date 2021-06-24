use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    action::{Action, ActionType, AppAction},
    camera::Camera,
    console::Console,
    draw::DrawSystem,
    entity::EntitySystem,
    geometry::{Position, Vec3f, Volume},
    input::{CameraInputContext, EventAction, InputSystem},
    time::Frame,
};

pub struct Application {
    pub window: Window,
    pub console: Console,
    pub draw_system: DrawSystem,
    pub entity_system: EntitySystem,
    pub input_system: InputSystem,
    pub close_requested: bool,
    fallback_camera: Camera,
    frame: Frame,
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
            close_requested: false,
            frame: Frame::new(),
        };

        let player_camera = app.entity_system.create();
        app.entity_system
            .cameras
            .set(player_camera, Camera::new(20.0, 0.1));
        app.entity_system.selected_camera = player_camera;

        let cube_friend = app.entity_system.create();
        app.entity_system.velocities.set(
            cube_friend,
            Vec3f {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
        );
        app.entity_system.positions.set(
            cube_friend,
            Position {
                x: 0.0,
                y: 20.0,
                z: 0.0,
            },
        );
        app.entity_system.colliders.set(
            cube_friend,
            Volume::Box {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
        );

        if let Some(action) = app
            .input_system
            .push_context(CameraInputContext::new(player_camera).into())
        {
            action.perform(&mut app);
        }

        app
    }

    pub fn receive_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        let action = self.process_app_event(event);
        match action {
            EventAction::Unconsumed => {
                if let EventAction::React(action) =
                    self.input_system.receive_event(&self.window.id(), event)
                {
                    action.perform(self);
                };
            }
            EventAction::Consumed => {
                // Nothing to do if event was consumed without producing an action
            }
            EventAction::React(action) => {
                action.perform(self);
            }
        }
        if self.close_requested {
            *control_flow = ControlFlow::Exit;
        }
    }

    /// Process all top-level events that drive basic application behavior.
    /// This includes window resizing and redraw requests, for example.
    fn process_app_event(&mut self, event: &Event<()>) -> EventAction {
        match event {
            Event::MainEventsCleared => {
                self.window.request_redraw();
                return EventAction::Consumed;
            }
            Event::RedrawRequested(window_id) if *window_id == self.window.id() => {
                self.redraw();
                return EventAction::Consumed;
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if self.window.id() == *window_id => match event {
                WindowEvent::CloseRequested => {
                    self.close_requested = true;
                    return EventAction::Consumed;
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.draw_system.resize_surface(new_inner_size);
                    return EventAction::Consumed;
                }
                WindowEvent::Resized(ref new_size) => {
                    self.draw_system.resize_surface(new_size);
                    return EventAction::Consumed;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let (Some(VirtualKeyCode::Tab), ElementState::Pressed) =
                        (input.virtual_keycode, input.state)
                    {
                        return EventAction::React(Action::App(AppAction::ToggleDebugOverlay));
                    }
                }
                _ => {}
            },
            _ => {}
        }
        EventAction::Unconsumed
    }

    fn redraw(&mut self) {
        self.frame.record();
        self.input_system
            .update(&self.frame, &mut self.entity_system);
        let camera = self
            .entity_system
            .get_selected_camera()
            .unwrap_or(&self.fallback_camera);
        self.draw_system.redraw(&self.frame, camera, &self.console);
    }
}
