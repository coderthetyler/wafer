use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use crate::{
    action::{Action, ActionType, ConfigAction},
    camera::Camera,
    entity::Ecs,
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
    pub hide_volumes: bool,
    pub should_exit: bool,
}

pub struct Application {
    pub config: AppConfig,
    pub window: Window,
    pub session: ConsoleSession,

    pub ecs: Ecs,

    pub movement_system: MovementSystem,
    pub puppet_system: PuppetSystem,
    pub physics_system: PhysicsSystem,

    pub paint_system: PaintSystem,
    pub events: EventInterpreter,

    pub frame: Frame,
}

impl Application {
    fn redraw(&mut self) {
        self.frame.start();
        self.paint_system
            .redraw(&self.config, &self.frame, &self.session, &self.ecs);
        self.physics_system.update(&self.frame, &mut self.ecs);
        self.movement_system.update(&self.frame, &mut self.ecs);
        self.puppet_system.update(&self.frame, &mut self.ecs);
        self.frame.end();
    }

    pub fn receive_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        let action = self.process_app_event(event);
        match action {
            EventAction::Unconsumed => {
                if let EventAction::React(action) = self.events.consume(&self.window.id(), event) {
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

    pub fn start(&mut self) {
        let player = self.ecs.pool.allocate();
        self.ecs.comps.camera.set(player, Camera::new());
        self.ecs.comps.position.set(
            player,
            Position {
                x: 0.0,
                y: 0.5,
                z: 10.0,
            },
        );
        self.ecs.comps.rotation.set(
            player,
            Rotation {
                pitch: 0.0,
                yaw: 180.0,
                roll: 0.0,
            },
        );
        self.ecs.comps.puppet.set(
            player,
            Puppet::FreeCamera(FreeCameraPuppet::new(30, Falloff::Geometric(1.8))),
        );
        self.paint_system.active_camera = player;

        let cube_friend_0 = self.ecs.pool.allocate();
        self.ecs.comps.velocity.set(
            cube_friend_0,
            Velocity {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
        );
        self.ecs.comps.position.set(
            cube_friend_0,
            Position {
                x: -2.0,
                y: -2.0,
                z: 0.0,
            },
        );
        self.ecs.comps.volumes.set(
            cube_friend_0,
            Volume::Box {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
        );
        self.ecs.comps.rotation.set(
            cube_friend_0,
            Rotation {
                pitch: 45.0,
                yaw: 45.0,
                roll: 45.0,
            },
        );
        self.ecs.comps.spin.set(
            cube_friend_0,
            Spin {
                x: 2.0,
                y: 4.5,
                z: 6.5,
            },
        );

        let cube_friend_1 = self.ecs.pool.allocate();
        self.ecs.comps.velocity.set(
            cube_friend_1,
            Velocity {
                x: -0.1,
                y: 0.0,
                z: 0.0,
            },
        );
        self.ecs.comps.position.set(
            cube_friend_1,
            Position {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
        );
        self.ecs.comps.volumes.set(
            cube_friend_1,
            Volume::Box {
                x: 0.75,
                y: 0.50,
                z: 0.75,
            },
        );
        if let Some(action) = self.events.push_context(SceneInputContext::new()) {
            action.perform(self);
        }
        self.window.set_cursor_visible(false);
    }

    pub async fn new(window: Window) -> Self {
        let paint_system = PaintSystem::new(&window).await;
        Self {
            config: AppConfig::default(),
            window,
            session: ConsoleSession::new(),

            ecs: Ecs::new(),

            paint_system,
            movement_system: MovementSystem::new(),
            puppet_system: PuppetSystem::new(),
            physics_system: PhysicsSystem::new(),

            events: EventInterpreter::new(),
            frame: Frame::new(60, Falloff::Geometric(1.1)),
        }
    }
}
