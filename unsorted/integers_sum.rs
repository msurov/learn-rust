use std::io;

#[allow(unused_parens)]
fn main() {
  let mut start = String::new();
  io::stdin().read_line(&mut start).expect("Ошибка чтения");
  let start: i32 = start.trim().parse().expect("Ошибка преобразования");

  let mut end = String::new();
  io::stdin().read_line(&mut end).expect("Ошибка чтения");
  let end: i32 = end.trim().parse().expect("Ошибка преобразования");

  let mut sum = 0;

  for i in (start..=end) {
    sum += i;
  }

  println!("{}", sum);
}
