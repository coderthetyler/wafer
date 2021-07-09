use app::Application;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod action;
mod app;
mod camera;
mod entity;
mod frame;
mod input;
mod movement;
mod paint;
mod physics;
mod puppet;
mod session;
mod types;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("voxel-planet")
        .build(&event_loop)
        .unwrap();

    let mut app = futures::executor::block_on(Application::new(window));
    app.start();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        app.receive_event(&event, control_flow);
    });
}
