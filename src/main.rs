mod structures;

fn main() {
  startup();
  let char1 = structures::Character::Character {
    name: "Character1".to_string(),
  };
  println!("{:?}",char1.name);
}

fn startup() {
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  println!("Grid Engine v{}",VERSION);
}
