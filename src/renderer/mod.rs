pub mod gpu;
pub mod physics;
pub mod draw;

use std::{sync::Arc, time::Instant};

use winit::{dpi::PhysicalSize, window::Window};

use gpu::*;
use physics::*;
use draw::*;

pub struct Renderer {
    pub gpu: Gpu,
    pub physics: Physics,

    pub last_frame: Instant,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let gpu = Gpu::new(window).await;

        Self {
            gpu,
            physics: Physics::new(),
            last_frame: Instant::now(),
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }

    pub fn render(&mut self) {
        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();

        self.physics.update(dt);

        draw_frame(&mut self.gpu, &self.physics);
    }
}