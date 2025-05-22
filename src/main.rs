mod camera;
mod input;
mod scene;
mod renderer;

use camera::Camera;
use input::handle_input;
use scene::Scene;
use renderer::render;

use sdl2::init;
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;
use std::time::Duration;

fn main() {
    let sdl        = init().unwrap();
    let video      = sdl.video().unwrap();
    let window     = video
        .window("Grid Engine 3D", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas     = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // capture mouse for relative motion & hide cursor
    let mouse = sdl.mouse();
    mouse.set_relative_mouse_mode(true);
    mouse.show_cursor(false);
    canvas.window_mut().set_grab(true);

    let mut camera = Camera::new();
    let scene      = Scene::cube();
    let fov        = 256.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseMotion { xrel, yrel, .. } => {
                    // rotate camera: xrel -> yaw, yrel -> pitch
                    camera.yaw   += xrel as f32 * camera.rot_speed;
                    camera.pitch -= yrel as f32 * camera.rot_speed;
                    // clamp pitch to straight up/down
                    camera.pitch = camera.pitch.clamp(
                        -std::f32::consts::FRAC_PI_2,
                         std::f32::consts::FRAC_PI_2,
                    );
                }
                _ => {}
            }
        }

        handle_input(&mut camera, &event_pump.keyboard_state());
        render(&mut canvas, &camera, &scene, fov);

        std::thread::sleep(Duration::from_millis(16));
    }
}
