use std::io;

fn get_name() -> String {
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Не могу прочитать имя");
  return name.trim().to_string();
}

fn greet(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let name : String = get_name();
  greet(&name);
}