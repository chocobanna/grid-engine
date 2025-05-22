use sdl2::{pixels::Color, event::Event};
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Grid Engine", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl.event_pump().unwrap();

    let mut offset = 0i16;
    let mut dir = 1i16;

    loop {
        for evt in events.poll_iter() {
            if let Event::Quit { .. } = evt {
                return;
            }
        }

        // update position
        offset += dir;
        if offset > 200 { dir = -1; }
        else if offset < -200 { dir = 1; }

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let xs = [100i16 + offset, 400i16 + offset, 250i16 + offset];
        let ys = [100i16, 100i16, 400i16];
        canvas.filled_polygon(&xs, &ys, Color::RGB(255, 255, 255)).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}
