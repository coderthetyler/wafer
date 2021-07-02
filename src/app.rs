use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    action::{Action, ActionType, ConfigAction},
    camera::Camera,
    entity::{EntityComponents, EntityPool},
    frame::Frame,
    input::{EventAction, SceneInputContext},
    movement::MovementSystem,
    paint::PaintSystem,
    physics::PhysicsSystem,
    puppet::{Puppet, PuppetSystem},
    session::ConsoleSession,
    types::{Falloff, Position, Rotation, Spin, Velocity, Volume},
};
use crate::{input::EventInterpreter, puppet::FreeCameraPuppet};

#[derive(Default)]
pub struct AppConfig {
    pub hide_debug_overlay: bool,
    pub show_collider_volumes: bool,
    pub should_exit: bool,
}

pub struct Application {
    pub config: AppConfig,
    pub window: Window,
    pub session: ConsoleSession,

    pub entities: EntityPool,
    pub components: EntityComponents,

    pub paint_system: PaintSystem,
    pub movement_system: MovementSystem,
    pub puppet_system: PuppetSystem,
    pub physics_system: PhysicsSystem,

    pub interpreter: EventInterpreter,

    pub frame: Frame,
}

impl Application {
    fn redraw(&mut self) {
        self.frame.start();
        self.paint_system.redraw(
            &self.config,
            &self.frame,
            &self.session,
            &self.entities,
            &self.components,
        );
        self.physics_system
            .update(&self.frame, &self.entities, &mut self.components);
        self.movement_system
            .update(&self.frame, &self.entities, &mut self.components);
        self.puppet_system
            .update(&self.frame, &self.entities, &mut self.components);
        self.frame.end();
    }

    pub fn receive_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        let action = self.process_app_event(event);
        match action {
            EventAction::Unconsumed => {
                if let EventAction::React(action) =
                    self.interpreter.consume(&self.window.id(), event)
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
        if self.config.should_exit {
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
                    self.config.should_exit = true;
                    return EventAction::Consumed;
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.paint_system.resize_surface(new_inner_size);
                    return EventAction::Consumed;
                }
                WindowEvent::Resized(ref new_size) => {
                    self.paint_system.resize_surface(new_size);
                    return EventAction::Consumed;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let (Some(VirtualKeyCode::Tab), ElementState::Pressed) =
                        (input.virtual_keycode, input.state)
                    {
                        return EventAction::React(Action::Config(
                            ConfigAction::ToggleDebugOverlay,
                        ));
                    }
                }
                _ => {}
            },
            _ => {}
        }
        EventAction::Unconsumed
    }

    pub async fn new(window: Window) -> Self {
        let paint_system = PaintSystem::new(&window).await;
        let mut app = Application {
            config: AppConfig::default(),
            window,
            session: ConsoleSession::new(),

            entities: EntityPool::new(),
            components: EntityComponents::new(),

            paint_system,
            movement_system: MovementSystem::new(),
            puppet_system: PuppetSystem::new(),
            physics_system: PhysicsSystem::new(),

            interpreter: EventInterpreter::new(),
            frame: Frame::new(60, Falloff::Geometric(1.1)),
        };

        let player = app.entities.allocate();
        app.components.camera.set(player, Camera::new());
        app.components.position.set(
            player,
            Position {
                x: 0.0,
                y: 0.0,
                z: 32.0,
            },
        );
        app.components.rotation.set(
            player,
            Rotation {
                pitch: 0.0,
                yaw: 180.0,
                roll: 0.0,
            },
        );
        app.components.puppet.set(
            player,
            Puppet::FreeCamera(FreeCameraPuppet::new(10, Falloff::Geometric(1.8))),
        );
        app.paint_system.active_camera = player;

        let cube_friend_0 = app.entities.allocate();
        app.components.velocity.set(
            cube_friend_0,
            Velocity {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
        );
        app.components.position.set(
            cube_friend_0,
            Position {
                x: 0.0,
                y: 20.0,
                z: 0.0,
            },
        );
        app.components.colliders.set(
            cube_friend_0,
            Volume::Box {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
        );
        app.components.rotation.set(
            cube_friend_0,
            Rotation {
                pitch: 45.0,
                yaw: 45.0,
                roll: 45.0,
            },
        );
        app.components.spin.set(
            cube_friend_0,
            Spin {
                x: 2.0,
                y: 4.5,
                z: 6.5,
            },
        );

        let cube_friend_1 = app.entities.allocate();
        app.components.velocity.set(
            cube_friend_1,
            Velocity {
                x: -0.1,
                y: 0.0,
                z: 0.0,
            },
        );
        app.components.position.set(
            cube_friend_1,
            Position {
                x: 8.0,
                y: 18.0,
                z: 0.0,
            },
        );
        app.components.colliders.set(
            cube_friend_1,
            Volume::Box {
                x: 0.75,
                y: 0.50,
                z: 0.75,
            },
        );

        if let Some(action) = app.interpreter.push_context(SceneInputContext::new()) {
            action.perform(&mut app);
        }

        app
    }
}
