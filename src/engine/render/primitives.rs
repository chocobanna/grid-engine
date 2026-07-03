use crate::engine::config::{HEIGHT, WIDTH};

pub(crate) fn draw_soft_circle(
    frame: &mut [u8],
    cx: i32,
    cy: i32,
    radius: i32,
    color: [u8; 4],
    strength: u8,
) {
    if radius <= 0 {
        return;
    }

    let r2 = radius * radius;

    for y in -radius..=radius {
        for x in -radius..=radius {
            let d2 = x * x + y * y;

            if d2 <= r2 {
                let dist = (d2 as f32).sqrt() / radius as f32;
                let power = ((1.0 - dist).clamp(0.0, 1.0) * strength as f32) as u8;

                add_pixel(
                    frame,
                    cx + x,
                    cy + y,
                    [
                        scale_u8(color[0], power),
                        scale_u8(color[1], power),
                        scale_u8(color[2], power),
                        0,
                    ],
                );
            }
        }
    }
}

pub(crate) fn draw_rect(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for py in y..y + height {
        for px in x..x + width {
            put_pixel(frame, px, py, color);
        }
    }
}

pub(crate) fn draw_rect_outline(frame: &mut [u8], x: i32, y: i32, width: i32, height: i32, color: [u8; 4]) {
    for px in x..x + width {
        put_pixel(frame, px, y, color);
        put_pixel(frame, px, y + height - 1, color);
    }

    for py in y..y + height {
        put_pixel(frame, x, py, color);
        put_pixel(frame, x + width - 1, py, color);
    }
}

pub(crate) fn draw_circle_outline(frame: &mut [u8], cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
    if radius <= 1 {
        return;
    }

    let outer = radius * radius;
    let inner = (radius - 1) * (radius - 1);

    for y in -radius..=radius {
        for x in -radius..=radius {
            let d = x * x + y * y;

            if d <= outer && d >= inner {
                put_pixel(frame, cx + x, cy + y, color);
            }
        }
    }
}

pub(crate) fn draw_ellipse(frame: &mut [u8], cx: i32, cy: i32, rx: i32, ry: i32, color: [u8; 4]) {
    if rx <= 0 || ry <= 0 {
        return;
    }

    for y in -ry..=ry {
        for x in -rx..=rx {
            let nx = x as f32 / rx as f32;
            let ny = y as f32 / ry as f32;

            if nx * nx + ny * ny <= 1.0 {
                put_pixel(frame, cx + x, cy + y, color);
            }
        }
    }
}

pub(crate) fn draw_line(frame: &mut [u8], mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: [u8; 4]) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        put_pixel(frame, x0, y0, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub(crate) fn put_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as u32;
    let y = y as u32;

    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    let index = ((y * WIDTH + x) * 4) as usize;

    frame[index] = color[0];
    frame[index + 1] = color[1];
    frame[index + 2] = color[2];
    frame[index + 3] = color[3];
}

pub(crate) fn add_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as u32;
    let y = y as u32;

    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    let index = ((y * WIDTH + x) * 4) as usize;

    frame[index] = frame[index].saturating_add(color[0]);
    frame[index + 1] = frame[index + 1].saturating_add(color[1]);
    frame[index + 2] = frame[index + 2].saturating_add(color[2]);
}

pub(crate) fn darken_pixel(frame: &mut [u8], x: i32, y: i32, amount: u8) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as u32;
    let y = y as u32;

    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    let index = ((y * WIDTH + x) * 4) as usize;

    frame[index] = frame[index].saturating_sub(amount);
    frame[index + 1] = frame[index + 1].saturating_sub(amount);
    frame[index + 2] = frame[index + 2].saturating_sub(amount);
}

pub(crate) fn clamp_color(value: i16) -> u8 {
    value.clamp(0, 255) as u8
}

pub(crate) fn scale_u8(value: u8, amount: u8) -> u8 {
    ((value as u16 * amount as u16) / 255) as u8
}

pub(crate) fn pseudo_random(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}
