use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

use crate::camera::FreeCamera;
use crate::draw::DrawSystem;
use crate::entity::EntitySystem;
use crate::input::InputSystem;
use crate::time::Timestamp;

mod camera;
mod draw;
mod entity;
mod generation;
mod geometry;
mod input;
mod mesh;
mod texture;
mod time;
mod voxel;

fn main() {
    use winit::event::Event;
    use winit::event_loop::EventLoop;

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("voxel-planet")
        .with_visible(false)
        .build(&event_loop)
        .unwrap();
    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);
    window.set_visible(true);

    let mut draw_system = futures::executor::block_on(DrawSystem::new(&window));
    let mut input_system = InputSystem::new();
    let mut entity_system = EntitySystem::new();
    let main_camera = entity_system.create();
    entity_system
        .cameras
        .set(main_camera, Box::new(FreeCamera::new(20.0)));

    let mut last_time = Timestamp::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if input_system.receive_events(&window.id(), &event) {
            return;
        }
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = Timestamp::now();
                let delta_time = now.delta(last_time);
                last_time = now;
                draw_system.redraw(&entity_system);
                input_system.update(&mut entity_system);
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if window.id() == window_id => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    draw_system.resize_surface(new_inner_size)
                }
                WindowEvent::Resized(ref new_size) => draw_system.resize_surface(new_size),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
