mod engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    engine::run()
}
