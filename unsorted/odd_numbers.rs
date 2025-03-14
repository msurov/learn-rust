use std::io;

fn main() {

  let mut num_inp = String::new();
  io::stdin().read_line(&mut num_inp).expect("Ошибка чтения строки");

  let num = num_inp.trim().parse::<i32>().expect("Ошибка преобразования в число");

  let result = match num {
    0 => "Число равно нулю",
    i if i % 2 == 0 => "Число четное",
    i if i32::abs(i) % 2 == 1 => "Число нечетное",
    _ => "Неизвестный вариант"
  };

  println!("{}", result);
}