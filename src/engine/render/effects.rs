use crate::engine::config::{HEIGHT, WIDTH};

use super::primitives::*;

pub fn draw_frame(frame: &mut [u8], player_x: i32, player_y: i32, time: f32, boost: f32) {
    draw_plasma_background(frame, time);
    draw_nebula_clouds(frame, time);
    draw_star_layers(frame, time);
    draw_perspective_grid(frame, time);
    draw_aurora_ribbons(frame, time);
    draw_orbiting_particles(frame, player_x, player_y, time);
    draw_energy_rings(frame, player_x, player_y, time, boost);
    draw_floating_tiles(frame, time);
    draw_player_trail(frame, player_x, player_y, time, boost);
    draw_player(frame, player_x, player_y, time, boost);
    draw_vignette(frame);
    draw_scanlines(frame, time);
    draw_hud(frame, boost);
}

fn draw_plasma_background(frame: &mut [u8], time: f32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let fx = x as f32;
            let fy = y as f32;

            let a = ((fx * 0.035 + time * 1.1).sin() * 18.0) as i16;
            let b = ((fy * 0.045 - time * 0.9).cos() * 16.0) as i16;
            let c = (((fx + fy) * 0.025 + time * 1.7).sin() * 14.0) as i16;

            let nx = fx / WIDTH as f32 - 0.5;
            let ny = fy / HEIGHT as f32 - 0.5;
            let dist = (nx * nx + ny * ny).sqrt();
            let glow = ((1.0 - dist).clamp(0.0, 1.0) * 38.0) as i16;

            put_pixel(
                frame,
                x as i32,
                y as i32,
                [
                    clamp_color(8 + a + glow / 4),
                    clamp_color(12 + b + glow / 3),
                    clamp_color(28 + a + b + c + glow),
                    255,
                ],
            );
        }
    }
}

fn draw_nebula_clouds(frame: &mut [u8], time: f32) {
    for i in 0..7 {
        let seed = i * 193 + 77;

        let cx = (pseudo_random(seed) % WIDTH) as i32;
        let cy = (pseudo_random(seed + 9) % HEIGHT) as i32;

        let drift_x = (time * 0.25 + i as f32).sin() * 24.0;
        let drift_y = (time * 0.18 + i as f32 * 0.7).cos() * 16.0;

        let radius = 32 + (pseudo_random(seed + 44) % 38) as i32;

        draw_soft_circle(
            frame,
            cx + drift_x as i32,
            cy + drift_y as i32,
            radius,
            [20, 30, 75, 255],
            18,
        );
    }
}

fn draw_star_layers(frame: &mut [u8], time: f32) {
    draw_star_layer(frame, time, 80, 9.0, [70, 90, 145, 255]);
    draw_star_layer(frame, time, 55, 22.0, [120, 155, 220, 255]);
    draw_star_layer(frame, time, 30, 45.0, [210, 230, 255, 255]);
}

fn draw_star_layer(frame: &mut [u8], time: f32, count: u32, speed: f32, color: [u8; 4]) {
    for i in 0..count {
        let seed = i * 131 + count * 17;

        let x = pseudo_random(seed) % WIDTH;
        let base_y = pseudo_random(seed + 99) % HEIGHT;

        let y = ((base_y as f32 + time * speed) as u32) % HEIGHT;

        put_pixel(frame, x as i32, y as i32, color);

        if speed > 30.0 {
            put_pixel(frame, x as i32, y as i32 - 1, [color[0] / 2, color[1] / 2, color[2], 255]);
        }
    }
}

fn draw_perspective_grid(frame: &mut [u8], time: f32) {
    let horizon = HEIGHT as i32 / 2 + 18;
    let scroll = (time * 36.0) % 16.0;

    for y in horizon..HEIGHT as i32 {
        let depth = (y - horizon) as f32 / (HEIGHT as f32 - horizon as f32);
        let spacing = (4.0 + depth * depth * 28.0).max(4.0);
        let line_hit = ((y as f32 + scroll) % spacing) < 1.0;

        if line_hit {
            let brightness = (35.0 + depth * 75.0) as u8;

            for x in 0..WIDTH as i32 {
                put_pixel(frame, x, y, [20, brightness / 2, brightness, 255]);
            }
        }
    }

    let cx = WIDTH as i32 / 2;

    for i in -12..=12 {
        let end_x = cx + i * 32;
        draw_line(frame, cx, horizon, end_x, HEIGHT as i32 - 1, [22, 55, 90, 255]);
    }

    draw_line(frame, 0, horizon, WIDTH as i32, horizon, [30, 80, 120, 255]);
}

fn draw_aurora_ribbons(frame: &mut [u8], time: f32) {
    for ribbon in 0..3 {
        let offset = ribbon as f32 * 1.8;

        for x in 0..WIDTH as i32 {
            let fx = x as f32;

            let y = 34.0
                + (fx * 0.035 + time * 1.3 + offset).sin() * 12.0
                + (fx * 0.012 - time * 0.8).cos() * 8.0
                + ribbon as f32 * 14.0;

            for thickness in -2i32..=2 {
                let fade = 5 - thickness.abs();

                add_pixel(
                    frame,
                    x,
                    y as i32 + thickness,
                    [
                        (8 * fade) as u8,
                        (16 * fade) as u8,
                        (28 * fade) as u8,
                        0,
                    ],
                );
            }
        }
    }
}

fn draw_orbiting_particles(frame: &mut [u8], player_x: i32, player_y: i32, time: f32) {
    for i in 0..18 {
        let angle = time * 1.9 + i as f32 * 0.7;
        let radius = 18.0 + (i % 5) as f32 * 5.0 + (time * 3.0 + i as f32).sin() * 2.5;

        let x = player_x + (angle.cos() * radius) as i32;
        let y = player_y + (angle.sin() * radius * 0.65) as i32;

        let color = if i % 3 == 0 {
            [255, 210, 90, 255]
        } else if i % 3 == 1 {
            [100, 190, 255, 255]
        } else {
            [190, 100, 255, 255]
        };

        draw_rect(frame, x - 1, y - 1, 3, 3, color);
    }
}

fn draw_energy_rings(frame: &mut [u8], player_x: i32, player_y: i32, time: f32, boost: f32) {
    for i in 0..5 {
        let base = 14 + i * 13;
        let pulse = ((time * 24.0 + i as f32 * 3.0) as i32).rem_euclid(13);
        let radius = base + pulse + (boost * 10.0) as i32;

        let color = match i {
            0 => [90, 170, 255, 255],
            1 => [70, 120, 220, 255],
            2 => [55, 85, 170, 255],
            3 => [45, 65, 125, 255],
            _ => [35, 48, 90, 255],
        };

        draw_circle_outline(frame, player_x, player_y, radius, color);
    }

    if boost > 0.4 {
        draw_soft_circle(
            frame,
            player_x,
            player_y,
            (34.0 + boost * 22.0) as i32,
            [40, 90, 160, 255],
            (boost * 24.0) as u8,
        );
    }
}

fn draw_floating_tiles(frame: &mut [u8], time: f32) {
    for i in 0..26 {
        let seed = i * 43 + 21;

        let base_x = (pseudo_random(seed) % WIDTH) as f32;
        let base_y = (pseudo_random(seed + 12) % HEIGHT) as f32;

        let drift_x = (time * 0.65 + i as f32).sin() * 14.0;
        let drift_y = (time * 0.45 + i as f32 * 0.5).cos() * 9.0;

        let x = (base_x + drift_x) as i32;
        let y = (base_y + drift_y) as i32;

        let size = 3 + (pseudo_random(seed + 77) % 9) as i32;

        draw_rect_outline(frame, x, y, size, size, [65, 88, 130, 255]);

        if i % 4 == 0 {
            draw_line(frame, x, y, x + size, y + size, [35, 55, 90, 255]);
        }
    }
}

fn draw_player_trail(frame: &mut [u8], player_x: i32, player_y: i32, time: f32, boost: f32) {
    let trail_count = 8 + (boost * 8.0) as i32;

    for i in 1..=trail_count {
        let t = i as f32 / trail_count as f32;

        let wobble_x = (time * 8.0 - i as f32 * 0.8).sin() * 6.0;
        let wobble_y = (time * 6.0 - i as f32 * 0.5).cos() * 4.0;

        let x = player_x - i * 3 + wobble_x as i32;
        let y = player_y + wobble_y as i32;

        let shade = ((1.0 - t) * 130.0) as u8;

        draw_rect(frame, x - 3, y - 3, 6, 6, [shade / 2, shade, 190, 255]);
    }
}

fn draw_player(frame: &mut [u8], x: i32, y: i32, time: f32, boost: f32) {
    let bob = (time * 8.0).sin() as i32;

    draw_ellipse(frame, x, y + 12, 14, 4, [3, 5, 10, 255]);

    let glow_radius = 14 + (boost * 8.0) as i32;
    draw_soft_circle(frame, x, y + bob, glow_radius, [120, 90, 20, 255], 25);

    draw_rect(frame, x - 8, y - 8 + bob, 16, 16, [205, 150, 45, 255]);
    draw_rect(frame, x - 5, y - 5 + bob, 10, 10, [255, 225, 105, 255]);
    draw_rect(frame, x - 2, y - 2 + bob, 4, 4, [255, 255, 220, 255]);

    draw_line(frame, x - 11, y + bob, x - 18, y + bob, [255, 210, 90, 255]);
    draw_line(frame, x + 11, y + bob, x + 18, y + bob, [255, 210, 90, 255]);
    draw_line(frame, x, y - 11 + bob, x, y - 18 + bob, [255, 210, 90, 255]);
    draw_line(frame, x, y + 11 + bob, x, y + 18 + bob, [255, 210, 90, 255]);

    if boost > 0.1 {
        draw_circle_outline(
            frame,
            x,
            y + bob,
            11 + (boost * 8.0) as i32,
            [255, 240, 160, 255],
        );
    }
}

fn draw_vignette(frame: &mut [u8]) {
    let cx = WIDTH as f32 / 2.0;
    let cy = HEIGHT as f32 / 2.0;

    for y in 0..HEIGHT as i32 {
        for x in 0..WIDTH as i32 {
            let dx = (x as f32 - cx) / cx;
            let dy = (y as f32 - cy) / cy;

            let dist = (dx * dx + dy * dy).sqrt();
            let darkness = ((dist - 0.45).max(0.0) * 70.0) as u8;

            if darkness > 0 {
                darken_pixel(frame, x, y, darkness);
            }
        }
    }
}

fn draw_scanlines(frame: &mut [u8], time: f32) {
    let bright_line = ((time * 55.0) as i32).rem_euclid(HEIGHT as i32);

    for y in 0..HEIGHT as i32 {
        if y % 4 == 0 {
            for x in 0..WIDTH as i32 {
                darken_pixel(frame, x, y, 10);
            }
        }
    }

    for dy in 0..3 {
        let y = bright_line + dy;

        for x in 0..WIDTH as i32 {
            add_pixel(frame, x, y, [5, 9, 14, 0]);
        }
    }
}

fn draw_hud(frame: &mut [u8], boost: f32) {
    draw_rect(frame, 6, 6, 132, 28, [7, 9, 15, 255]);
    draw_rect_outline(frame, 6, 6, 132, 28, [75, 95, 135, 255]);

    draw_rect(frame, 14, 14, 72, 4, [30, 50, 65, 255]);
    draw_rect(frame, 14, 14, 72, 4, [90, 200, 125, 255]);

    draw_rect(frame, 14, 23, 72, 4, [30, 45, 70, 255]);
    draw_rect(frame, 14, 23, (72.0 * boost.max(0.2)) as i32, 4, [90, 150, 255, 255]);

    draw_rect(frame, 98, 13, 8, 8, [235, 175, 65, 255]);
    draw_rect(frame, 112, 13, 8, 8, [80, 170, 235, 255]);
    draw_rect(frame, 126, 13, 8, 8, [180, 100, 235, 255]);
}
