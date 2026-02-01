use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Creates an event loop and a window. On Wayland, winit will use it automatically if available.
    let event_loop = EventLoop::new().expect("failed to create event loop");

    let window = WindowBuilder::new()
        .with_title("winit on Wayland (NixOS)")
        .with_inner_size(LogicalSize::new(800.0, 450.0))
        .build(&event_loop)
        .expect("failed to create window");

    event_loop.run(move |event, elwt| {
        // Keep the loop running until we say otherwise.
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    // Just to prove events are flowing.
                    println!("resized: {}x{}", size.width, size.height);
                    window.request_redraw();
                }
                _ => {}
            },
            Event::AboutToWait => {
                // If you wanted animation, you’d request redraws here.
            }
            _ => {}
        }
    }).expect("event loop crashed");
}

