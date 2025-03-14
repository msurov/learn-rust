
#[allow(unused)]
fn unpack_tuples_test() {
  let x = (1, 2, 3);

  if let (1, a, b) = x {
    println!("1: {a}, {b}");
  }

  if let (2, a, b) = x {
    println!("2: {a}, {b}");
  }

  // fails:
  // if let (3, a, b, c) = x {
  //   println!("3: {a}, {b}, {c}");
  // }
}


#[allow(unused)]
fn unpack_refs() {
  let mut x = (1, "abc", 3.0);
  if let (1, ref x1, ref x2) = x {
    println!("1, {x1}, {x2}");
  }
  match x {
    (2, ref x1, ref x2) => println!("2, {x1}, {x2}"),
    (3, ref x1, ref x2) => println!("3, {x1}, {x2}"),
    _ => println!("undef")
  }
}

fn main() {
  let a : (u8, f32, &str) = (25, 1.0, "Max");
  let b : (u8, f32, &str) = (25, 1.1, "Elena");
  let mut c : (u8, f32, &str) = (33, 1.1, "Elena");

  if a >= c {
    println!("a >= c");
  }

  c.1 = 3.3;
  c.0 = 66;

  let (c0, c1, c2) = c;
  println!("c.0 = {c0}, c.1 = {c1}, c.2 = {c2}");

  if a > b {
    println!("a > b");
  } else if b > a {
    println!("b > a");
  } else {
    println!("a == b");
  }
}