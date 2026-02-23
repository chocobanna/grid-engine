use std::sync::Arc;

use super::{config::Config, renderer::Renderer};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    config: Config,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            window: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                el.create_window(
                    Window::default_attributes()
                        .with_title(self.config.title.clone())
                        .with_inner_size(PhysicalSize::new(
                            self.config.width,
                            self.config.height,
                        )),
                )
                .unwrap(),
            );

            let renderer =
                pollster::block_on(Renderer::new(window.clone()));

            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn window_event(
        &mut self,
        el: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => el.exit(),

            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(size);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(r) = &mut self.renderer {
                    r.render();
                }
            }

            _ => {}
        }

        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}