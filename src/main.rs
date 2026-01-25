use crate::structures::character::character;

fn main() {
  startup();
  let char1 = character {
    name: "Character1".to_string(),
  }
  println!("{:?}",char1);
}

fn startup() {
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  println!("Grid Engine v{}",VERSION);

}
