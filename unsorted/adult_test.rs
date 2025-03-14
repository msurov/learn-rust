
fn is_adult(age: i32) -> bool {
  age >= 18
}

fn main() {
  let mut age_str = String::new();
  std::io::stdin().read_line(&mut age_str).unwrap();
  let age: i32 = age_str.trim().parse().unwrap();

  if is_adult(age) {
      println!("Совершеннолетний");
  } else {
      println!("Несовершеннолетний");
  }
}
