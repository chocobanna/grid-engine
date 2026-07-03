use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::engine::config::{HEIGHT, SCALE, WIDTH, WINDOW_TITLE};

pub struct WindowResources {
    pub window: Arc<Window>,
    pub pixels: Pixels<'static>,
}

pub fn create_window(event_loop: &ActiveEventLoop) -> WindowResources {
    let window_size = LogicalSize::new((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64);

    let window = Arc::new(
        event_loop
            .create_window(
                Window::default_attributes()
                    .with_title(WINDOW_TITLE)
                    .with_inner_size(window_size)
                    .with_min_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64)),
            )
            .expect("failed to create window"),
    );

    let size = window.inner_size();
    let surface = SurfaceTexture::new(size.width.max(1), size.height.max(1), window.clone());

    let pixels = Pixels::new(WIDTH, HEIGHT, surface).expect("failed to create pixels renderer");

    WindowResources { window, pixels }
}
