use softbuffer_quickstart::{SoftbufferWindow, WindowProperties};
use winit::event::{WindowEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a softbuffer window
    let mut window = SoftbufferWindow::new(WindowProperties::default());

    window.run(move |window, event| {
        match event {
            WindowEvent::RedrawRequested => {
                // Get size
                let (width, height) = window.inner_size();

                // Get mutable reference to lived buffer
                let mut buf = window.buffer_mut();
                // Prepare and draw into buffer
                for y in 0..height {
                    for x in 0..width {
                        let idx = (y * width + x) as usize;
                        buf[idx] = 0xFF000000; // black
                    }
                }

                // Rasterize a simple triangle
                let tri = [
                    ((width / 2) as i32, (height / 4) as i32),
                    ((width / 4) as i32, (3 * height / 4) as i32),
                    ((3 * width / 4) as i32, (3 * height / 4) as i32),
                ];

                for y in 0..height as i32 {
                    for x in 0..width as i32 {
                        let (u, v, w) = barycentric((x, y), tri[0], tri[1], tri[2]);
                        if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                            let idx = (y as usize * width as usize + x as usize);
                            buf[idx] = 0xFFFFFFFF; // white
                        }
                    }
                }
            }

            WindowEvent::CloseRequested => {
                // Nothing to call — just return and loop will end
            }

            _ => {}
        }
    })?;

    Ok(())
}

fn barycentric(p: (i32, i32), a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> (f32, f32, f32) {
    let (px, py) = (p.0 as f32, p.1 as f32);
    let (ax, ay) = (a.0 as f32, a.1 as f32);
    let (bx, by) = (b.0 as f32, b.1 as f32);
    let (cx, cy) = (c.0 as f32, c.1 as f32);

    let den = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
    if den.abs() < f32::EPSILON {
        return (-1.0, -1.0, -1.0);
    }
    let u = ((by - cy) * (px - cx) + (cx - bx) * (py - cy)) / den;
    let v = ((cy - ay) * (px - cx) + (ax - cx) * (py - cy)) / den;
    let w = 1.0 - u - v;
    (u, v, w)
}
