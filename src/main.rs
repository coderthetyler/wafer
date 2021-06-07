use draw::State;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

mod camera;
mod draw;
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
    let mut state = futures::executor::block_on(State::new(&window));
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if state.input(&window.id(), &event) {
            return;
        }
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                state.redraw();
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if window.id() == window_id => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(new_inner_size)
                }
                WindowEvent::Resized(ref new_size) => state.resize(new_size),
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
