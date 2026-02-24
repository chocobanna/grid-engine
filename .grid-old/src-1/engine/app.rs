use std::sync::Arc;

use super::{renderer::Renderer, config::Config, sprite::Sprite};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    config: Config,
    sprites: Vec<Sprite>,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl App {

    pub fn new(config: Config, sprites: Vec<Sprite>) -> Self {
        Self {
            config,
            sprites,
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
                ).unwrap()
            );

            let renderer =
                pollster::block_on(Renderer::new(window.clone(), &self.sprites));

            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn window_event(
        &mut self,
        el: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent
    ) {

        match event {

            WindowEvent::CloseRequested => el.exit(),

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