// src/renderer.rs
use glam::Vec3;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::{camera::Camera, scene::Scene};

pub fn render(canvas: &mut Canvas<Window>, camera: &Camera, scene: &Scene, fov: f32) {
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    let (w, h) = canvas.window().size();
    let rot = camera.rotation();

    let mut proj: Vec<Option<Point>> = vec![None; scene.verts.len()];
    for (i, &v) in scene.verts.iter().enumerate() {
        let rel = v - camera.pos;
        let p   = rot * rel;
        if p.z > 0.1 {
            let sx = p.x * fov / p.z + w as f32 / 2.0;
            let sy = -p.y * fov / p.z + h as f32 / 2.0;
            proj[i] = Some(Point::new(sx as i32, sy as i32));
        }
    }

    canvas.set_draw_color(Color::RGB(255,255,255));
    for &(i0, i1) in &scene.edges {
        if let (Some(a), Some(b)) = (proj[i0], proj[i1]) {
            canvas.draw_line(a, b).unwrap();
        }
    }

    canvas.present();
}
