mod engine;

use engine::*;

fn main() {
    Engine::new()
        .title("Multiple Shapes Demo")
        .size(900, 700)

        // triangle (left)
        .sprite(
            Sprite {
                vertices: vec![
                    [0.0, 0.5],
                    [-0.5, -0.5],
                    [0.5, -0.5],
                ],
                position: [-0.8, 0.5],
                rotation: 0.0,
                scale: 0.25,
                color: [1.0, 0.2, 0.2],
                velocity: [0.0, 0.0],
            },
        )

        // square (right)
        .sprite(
            Sprite {
                vertices: vec![
                    [-0.5, 0.5],
                    [0.5, 0.5],
                    [0.5, -0.5],
                    [-0.5, -0.5],
                ],
                position: [0.8, 0.5],
                rotation: 0.0,
                scale: 0.2,
                color: [0.2, 0.8, 0.3],
                velocity: [0.0, 0.0],
            },
        )

        // pentagon (bottom)
        .sprite(
            Sprite {
                vertices: vec![
                    [0.0, 0.6],
                    [0.57, 0.18],
                    [0.35, -0.5],
                    [-0.35, -0.5],
                    [-0.57, 0.18],
                ],
                position: [0.0, -0.7],
                rotation: 0.0,
                scale: 0.25,
                color: [0.3, 0.4, 1.0],
                velocity: [0.0, 0.0],
            },
        )

        .run();
}