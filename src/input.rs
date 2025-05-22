use crate::camera::Camera;
use sdl2::keyboard::{Scancode, KeyboardState};

pub fn handle_input(camera: &mut Camera, keys: &KeyboardState) {
    let fwd   = camera.forward();
    let right = camera.right();

    if keys.is_scancode_pressed(Scancode::W) {
        camera.pos += fwd * camera.speed;
    }
    if keys.is_scancode_pressed(Scancode::S) {
        camera.pos -= fwd * camera.speed;
    }
    if keys.is_scancode_pressed(Scancode::A) {
        camera.pos -= right * camera.speed;
    }
    if keys.is_scancode_pressed(Scancode::D) {
        camera.pos += right * camera.speed;
    }

    // vertical movement
    if keys.is_scancode_pressed(Scancode::LCtrl) || keys.is_scancode_pressed(Scancode::RCtrl) {
        camera.pos.y -= camera.speed;
    }
    if keys.is_scancode_pressed(Scancode::Space) {
        camera.pos.y += camera.speed;
    }
}
