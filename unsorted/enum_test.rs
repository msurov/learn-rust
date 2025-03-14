
#[allow(dead_code)]
enum MathExpr {
  Add(i32, i32),
  Sub(i32, i32),
  Mul(i32, i32),
  Div(i32, i32),
  Sq(i32),
}

fn print_math_expr(expr : &MathExpr) {
  match *expr {
    MathExpr::Add(a, b) => println!("{a} + {b}"),
    MathExpr::Sub(a, b) => println!("{a} - {b}"),
    MathExpr::Mul(a, b) => println!("{a} * {b}"),
    MathExpr::Div(a, b) => println!("{a} / {b}"),
    MathExpr::Sq(a) => println!("{a}^2"),
  }
}

fn compute(expr : &MathExpr) -> i32 {
  return match *expr {
    MathExpr::Add(a, b) => a + b,
    MathExpr::Sub(a, b) => a - b,
    MathExpr::Mul(a, b) => a * b,
    MathExpr::Div(a, b) => a / b,
    MathExpr::Sq(a) => a * a
  }
}

fn main() {
  let expr : MathExpr = MathExpr::Sq(5);

  // option 1
  if false {
    print_math_expr(&expr);
    let res = compute(&expr);
    println!("result is {res}");
   }

  // option 2
  if true {
    if let MathExpr::Sq(value) = expr {
      println!("{}^2 = {}", value, value * value);
    } else if let MathExpr::Mul(a, b) = expr {
      println!("{} * {} = {}", a, b, a * b);
    } else if let MathExpr::Add(a, b) = expr {
      println!("{} * {} = {}", a, b, a + b);
    }
  }
}
