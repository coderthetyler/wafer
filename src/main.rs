use app::Application;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod action;
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

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
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
