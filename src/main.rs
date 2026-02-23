mod app;

use app::App;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // winit 0.30 uses the new application-style event loop,
    // but you can still do a straightforward run with a closure.
    let event_loop = EventLoop::new().expect("failed to create event loop");

    let window = WindowBuilder::new()
        .with_title("winit 0.30 + wgpu 28")
        .build(&event_loop)
        .expect("failed to create window");

    // wgpu init is async; pollster blocks until done.
    let mut app = pollster::block_on(App::new(&window));

    event_loop
        .run(move |event, elwt| {
            // Keep the app alive unless we decide otherwise.
            elwt.set_control_flow(ControlFlow::Poll);

            app.handle_event(&window, event, elwt);
        })
        .expect("event loop crashed");
}