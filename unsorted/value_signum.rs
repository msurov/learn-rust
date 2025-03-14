use std::io;

fn main() {
  let mut number = String::new();
  io::stdin().read_line(&mut number).expect("Ошибка чтения строки");
  let number: i32 = number.trim().parse::<i32>().expect("Ошибка преобразования в число");

  let result = match number {
    0 => "Ноль",
    (1..) => "Положительное число",
    (..0) => "Отрицательное число"
  };

  println!("Результат: {}", result);
}
