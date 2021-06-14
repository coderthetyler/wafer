use app::Application;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::Window;

use crate::camera::Camera;
use crate::draw::DrawSystem;
use crate::entity::EntitySystem;
use crate::input::CameraInputContext;
use crate::input::InputSystem;
use crate::time::Timestamp;

mod app;
mod camera;
mod console;
mod draw;
mod entity;
mod generation;
mod geometry;
mod input;
mod planets;
mod texture;
mod time;

/*
Console TODO

- open console on 't'
- draw console text
- update console text on key presses
- submit on Enter
- navigate on up/down
*/

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("voxel-planet")
        .with_visible(false)
        .build(&event_loop)
        .unwrap();
    window.set_visible(true);

    let mut app = futures::executor::block_on(Application::new(window));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        app.receive_event(&event, control_flow);
    });
}
