pub mod app;
pub mod config;
pub mod input;
pub mod render;
pub mod window;

pub use self::app::App;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = winit::event_loop::EventLoop::new()?;

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
