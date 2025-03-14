
fn compare_numbers(a: i32, b: i32) {
  if a > b {
    println!("Первое число больше");
  } else if b > a {
    println!("Второе число больше");
  } else {
    println!("Числа равны");
  }
}

fn main() {
  let mut num1_str = String::new();
  let mut num2_str = String::new();

  std::io::stdin().read_line(&mut num1_str).expect("Couldn't read the first number");
  std::io::stdin().read_line(&mut num2_str).expect("Couldn't read the second number");

  let num1: i32 = num1_str.trim().parse().expect("Couldn't parse the first number");
  let num2: i32 = num2_str.trim().parse().expect("Couldn't parse the second number");

  compare_numbers(num1, num2);
}
