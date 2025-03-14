
fn do_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) {
  let result = operation(a, b); // Вызов переданной функции
  println!("result: {}", result);
}

fn multiply(a: i32, b: i32) -> i32 {
  a * b
}

fn main() {
  let sum: fn(i32, i32) -> i32 = |a, b| a + b; // Определение анонимной функции sum

  do_operation(6, 4, sum); // Передача sum в качестве аргумента
  do_operation(6, 4, multiply); // Передача multiply в качестве аргумента
}
