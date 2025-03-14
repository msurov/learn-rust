
enum MathOperations {
  Add, // assumed = 0
  Mul, // assumed = 1
  Div, // etc
  Sub,
  Pow,
  Sq
}

#[allow(unused_assignments)]
fn main() {
  assert_eq!(MathOperations::Add as i32, 0);
  assert_eq!(MathOperations::Mul as i32, 1);
  assert_eq!(MathOperations::Div as i32, 2);
  assert_eq!(MathOperations::Sub as i32, 3);
  assert_eq!(MathOperations::Pow as i32, 4);
  assert_eq!(MathOperations::Sq as i32, 5);

  let mut op = MathOperations::Add;
  op = MathOperations::Mul;
  println!("op = {}", op as u32);
}
