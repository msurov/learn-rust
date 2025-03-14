use std::io;

fn print_participant((name, age, city) : (&str, u32, &str)) {
  println!("Участник: {name}, {age}, {city}");
}

fn main() {
  let mut name = String::new();
  let mut age = String::new();
  let mut city = String::new();

  io::stdin().read_line(&mut name).expect("Can't read name");
  let name = name.trim();
  io::stdin().read_line(&mut age).expect("Can't read age");
  let age = age.trim().parse::<u32>().expect("Can't parse age");
  io::stdin().read_line(&mut city).expect("Can't read city");
  let city = city.trim();

  let participant : (&str, u32, &str) = (name, age, city);
  print_participant(participant);
}