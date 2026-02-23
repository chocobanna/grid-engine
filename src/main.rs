mod engine;

use engine::*;

fn main() {
    Engine::new()
        .title("Sprite Demo")
        .size(800, 600)
        .sprite(
            Sprite::triangle()
                .position(0.0, 0.0)
                .scale(0.4)
                .color(1.0, 0.4, 0.2)
        )
        .run();
}