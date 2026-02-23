mod app;
mod renderer;
mod config;

use config::Config;
use winit::event_loop::EventLoop;

pub struct Engine {
    config: Config,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.config.title = title.to_string();
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();

        let mut app = app::App::new(self.config);

        event_loop.run_app(&mut app).unwrap();
    }
}