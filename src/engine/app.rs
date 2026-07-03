use std::sync::Arc;
use std::time::Instant;

use pixels::Pixels;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::engine::{
    config::{HEIGHT, PLAYER_SPEED, WIDTH},
    input::InputState,
    render,
    window as engine_window,
};

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,

    input: InputState,

    started: Instant,
    last_frame: Instant,

    player_x: f32,
    player_y: f32,

    boost_flash: f32,
}

impl App {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            window: None,
            pixels: None,

            input: InputState::default(),

            started: now,
            last_frame: now,

            player_x: WIDTH as f32 / 2.0,
            player_y: HEIGHT as f32 / 2.0,

            boost_flash: 0.0,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let resources = engine_window::create_window(event_loop);

        self.window = Some(resources.window);
        self.pixels = Some(resources.pixels);
    }

    fn set_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::Escape if pressed => {
                event_loop.exit();
            }

            KeyCode::KeyW | KeyCode::ArrowUp => self.input.up = pressed,
            KeyCode::KeyS | KeyCode::ArrowDown => self.input.down = pressed,
            KeyCode::KeyA | KeyCode::ArrowLeft => self.input.left = pressed,
            KeyCode::KeyD | KeyCode::ArrowRight => self.input.right = pressed,

            KeyCode::Space if pressed => {
                self.reset_player();
            }

            _ => {}
        }
    }

    fn reset_player(&mut self) {
        self.player_x = WIDTH as f32 / 2.0;
        self.player_y = HEIGHT as f32 / 2.0;
        self.started = Instant::now();
        self.boost_flash = 1.0;
    }

    fn update(&mut self, dt: f32) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if self.input.left {
            dx -= 1.0;
        }

        if self.input.right {
            dx += 1.0;
        }

        if self.input.up {
            dy -= 1.0;
        }

        if self.input.down {
            dy += 1.0;
        }

        let len = (dx * dx + dy * dy).sqrt();

        if len > 0.0 {
            dx /= len;
            dy /= len;

            self.player_x += dx * PLAYER_SPEED * dt;
            self.player_y += dy * PLAYER_SPEED * dt;
            self.boost_flash = (self.boost_flash + dt * 2.0).min(1.0);
        } else {
            self.boost_flash -= dt * 1.8;
        }

        self.boost_flash = self.boost_flash.clamp(0.0, 1.0);

        self.player_x = self.player_x.clamp(10.0, WIDTH as f32 - 10.0);
        self.player_y = self.player_y.clamp(10.0, HEIGHT as f32 - 10.0);
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let mut dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        dt = dt.min(1.0 / 20.0);

        self.update(dt);

        let Some(window) = self.window.as_ref() else {
            return;
        };

        let Some(pixels) = self.pixels.as_mut() else {
            return;
        };

        let time = self.started.elapsed().as_secs_f32();

        render::draw_frame(
            pixels.frame_mut(),
            self.player_x as i32,
            self.player_y as i32,
            time,
            self.boost_flash,
        );

        if let Err(err) = pixels.render() {
            eprintln!("render failed: {err}");
            event_loop.exit();
            return;
        }

        window.request_redraw();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.create_window(event_loop);
        }

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    if let Err(err) = pixels.resize_surface(size.width.max(1), size.height.max(1)) {
                        eprintln!("resize failed: {err}");
                        event_loop.exit();
                    }
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;

                if let PhysicalKey::Code(key) = event.physical_key {
                    self.set_key(event_loop, key, pressed);
                }
            }

            WindowEvent::RedrawRequested => {
                self.redraw(event_loop);
            }

            _ => {}
        }
    }
}
