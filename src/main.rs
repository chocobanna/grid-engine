fn main() {
  startup();
}

fn startup() {
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  println!("Grid Engine v{}",VERSION);

}
