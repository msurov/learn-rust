use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
  let n: u32 = input.trim().parse().expect("Ошибка преобразования в число");

  let mut factorial = 1;

  for i in 1..=n {
    factorial *= i;
  }

  println!("Факториал числа {} равен {}", n, factorial);
}