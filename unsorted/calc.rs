use std::io;

fn add(a : i32, b : i32) -> i32 {
  a + b
}

fn subtract(a : i32, b : i32) -> i32 {
  a - b
}

fn multiply(a : i32, b : i32) -> i32 {
  a * b
}

fn divide(a : i32, b : i32) -> i32 {
  a / b
}

fn stub(_a : i32, _b : i32) -> i32 {
  0
}

#[allow(unpredictable_function_pointer_comparisons)]
fn main() {
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Не удалось прочитать первый аргумент");
  let a : i32 = a.trim().parse::<i32>().expect("Не удалось преобразовать первый аргумент к числу");

  let mut op = String::new();
  io::stdin().read_line(&mut op).expect("Не удалось прочитать операцию");
  let op = op.trim();

  let fun : fn(i32, i32) -> i32 = match op {
    "+" => add,
    "-" => subtract,
    "*" => multiply,
    "/" => divide,
    _ => stub
  };

  if fun == stub {
    println!("undefined operation \"{op}\"");
    return;
  }

  let mut b = String::new();
  io::stdin().read_line(&mut b).expect("Не удалось прочитать второй аргумент");
  let b : i32 = b.trim().parse::<i32>().expect("Не удалось преобразовать второй аргумент к числу");

  println!("{a} {op} {b} = {}", fun(a, b));
}
