mod engine;

use engine::Engine;

fn main() {
    Engine::new()
        .title("Triangle Demo")
        .size(800, 600)
        .run();
}