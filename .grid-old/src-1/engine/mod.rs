mod app;
mod renderer;
mod sprite;
mod config;

pub use sprite::Sprite;

use config::Config;
use winit::event_loop::EventLoop;

pub struct Engine {
    config: Config,
    sprites: Vec<Sprite>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            sprites: vec![],
        }
    }

    pub fn title(mut self, t: &str) -> Self {
        self.config.title = t.to_string();
        self
    }

    pub fn size(mut self, w: u32, h: u32) -> Self {
        self.config.width = w;
        self.config.height = h;
        self
    }

    pub fn sprite(mut self, sprite: Sprite) -> Self {
        self.sprites.push(sprite);
        self
    }

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        let mut app = app::App::new(self.config, self.sprites);

        event_loop.run_app(&mut app).unwrap();
    }
}