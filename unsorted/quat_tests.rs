#![allow(dead_code)]
#![allow(unused)]

mod quat;
use num::{Rational64,};
use quat::Quat;


#[test]
fn test_print() {
  let a: Quat<f64> = Quat {w : 1., x : 2., y : 3., z : 4. };
  let b = Quat {w : 1., x : 2., y : 3., z : 4. };
  let c = a * b;
  let d = a / b;
  println!("{} o {} = {}", a, b, c);
  println!("{} / {} = {}", a, b, d);
  let z = -a;
  println!("-{} = {}", a, z);
  println!("||{}|| = {}", z, z.norm());
  println!("{}* = {}", a, a.conj());
}

#[test]
fn test_conj() {
  let a: Quat<i32> = Quat {w : 1, x : -2, y : 3, z : -4 };
  let b: Quat<i32> = Quat {w : 1, x : 2, y : -3, z : 4 };
  assert_eq!(a, b.conj());
}

#[test]
fn test_sub() {
  let a: Quat<i32> = Quat {w : 1, x : -2, y : 3, z : -4 };
  let b: Quat<i32> = Quat {w : 2, x : 1, y : 9, z : 0 };
  let c = a + b;
  let d = a - (-b);
  assert_eq!(c, d);
}

#[test]
fn test_mul() {
  let a: Quat<i32> = Quat {w : 1, x : -2, y : 3, z : -4 };
  assert_eq!(Quat::make_scal(5) * a, a.mul_num(5));
}

#[test]
fn test_cmp() {
  let a: Quat<i32> = Quat {w : 1, x : -2, y : 3, z : -4 };
  let b: Quat<i32> = Quat {w : 2, x : 1, y : 9, z : 0 };
  assert_ne!(a, b);
}

#[test]
fn test_gaussian_numbers() {
  let _0x3 = [Rational64::new(0, 1); 3];
  let _1 = Rational64::new(1, 1);

  let a = Quat { 
    w : Rational64::new(1, 5),
    x : Rational64::new(2, 5),
    y : Rational64::new(3, 5),
    z : Rational64::new(4, 5),
  };
  let a_inv = a.inv();
  let z = a * a_inv;
  assert_eq!(z.w, _1);
  assert_eq!(z.vec(), _0x3);

  let a_conj = a.conj();
  let z = a * a_conj;
  assert_eq!(z.vec(), _0x3);

  let z = a / a;
  assert_eq!(z.w, Rational64::new(1, 1));
  assert_eq!(z.vec(), _0x3);
}

fn main() {
}
