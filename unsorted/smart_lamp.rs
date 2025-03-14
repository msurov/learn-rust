
fn in_range(val : i32, from : i32, to : i32) -> bool {
  val >= from && val <= to
}

fn main() {
  let mut inp_str = String::new();
  std::io::stdin().read_line(&mut inp_str).expect("Couldn't read current time");
  let current_hour: i32 = inp_str.trim().parse::<i32>().expect("Couldn't parse current time");

  let action = if in_range(current_hour, 8, 21) {
    "Включить"
  } else if in_range(current_hour, 22, 23) || in_range(current_hour, 0, 5) {
    "Выключить"
  } else if in_range(current_hour, 6, 7) {
    "Рассвет"
  } else {
    "Ошибка"
  };

  println!("Действие умной лампы: {}", action);
}
