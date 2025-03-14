

fn main() {
  let mut age_inp = String::new();
  std::io::stdin().read_line(&mut age_inp).expect("Coulnd't read age");
  let age = age_inp.trim().parse::<u32>().expect("Couldn't parse age");

  let mut role_inp = String::new();
  std::io::stdin().read_line(&mut role_inp).expect("Couldn't read role");
  let role = role_inp.trim();

  if role == "admin" {
    println!("Доступ разрешен");
  } else if role == "user" && age >= 18 {
    println!("Доступ разрешен");
  } else {
    println!("Доступ запрещен");
  }
}
