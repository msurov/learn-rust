use std::{fmt::Display, cmp::PartialOrd};
use num::{Num, Float};

fn maxarg<T>(args : &[T]) -> &T where T : PartialOrd {
  assert!(args.len() > 0);
  let mut maxval = &args[0];
  for e in args {
    if *e > *maxval {
      maxval = e;
    }
  }
  return maxval;
}

fn minarg<T>(args : &[T]) -> &T where T : PartialOrd {
  assert!(args.len() > 0);
  let mut minval = &args[0];
  for e in args {
    if *e < *minval {
      minval = e;
    }
  }
  return minval;
}

/**
 * Struct Vector3: declaration
 */
struct Vector3<T> where T : Num {
  x : T,
  y : T,
  z : T
}

/**
 * Struct Vector3: implement methods new, max, min
 */
impl<T> Vector3<T> where T : Num + Copy {

  fn new(x : T, y : T, z : T) -> Self {
    Vector3 {x, y, z}
  }

  fn sq(&self) -> T {
    self.dot(self)
  }

  fn dot(&self, other : &Self) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  fn elementwise_op(&self, other : &Self, op : fn(&T, &T) -> T) -> Self {
    Vector3 {
      x : op(&self.x, &other.x),
      y : op(&self.y, &other.y),
      z : op(&self.z, &other.z)
    }
  }

  fn sub(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a, b| *a - *b)
  }

  fn add(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a, b| *a + *b)
  }

  fn mul(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a, b| (*a) * (*b))
  }

  fn div(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a, b| (*a) / (*b))
  }
}

impl<T> Vector3<T> where T : Num + Copy + PartialOrd {

  fn max(&self) -> T {
    return *maxarg(&[self.x, self.y, self.z]);
  }

  fn min(&self) -> T {
    return *minarg(&[self.x, self.y, self.z]);
  }
}

impl<T> Vector3<T> where T : Float + Copy {

  fn norm(&self) -> T {
    Float::sqrt(self.sq())
  }

  fn dist(&self, other : &Self) -> T {
    self.sub(other).norm()
  }
}

/**
 * Struct Vector3: implement trait Display
 */
impl<T> Display for Vector3<T> where T : Num + Display {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vec({}, {}, {})", self.x, self.y, self.z)
  }
}

/**
 * Create new trait
 */
trait CrossProduct {
  fn cross(&self, other : &Self) -> Self;
}

/**
 * Struct Vector3: implement the trait CrossProduct
 */
impl<T> CrossProduct for Vector3<T> where T : Num + Copy {
  fn cross(&self, other : &Self) -> Vector3<T> {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;
    Vector3{ x, y, z }
  }
}

fn main() {
  let pt = Vector3::<i32>::new(1, 2, 3);
  println!("Max component of {} is {}", pt, pt.max());
  println!("Min component of {} is {}", pt, pt.min());
  println!("pt = {}", pt);

  let a = Vector3::new(1., 2., 3.);
  let b = Vector3::new(3., -1., 4.);
  let c = a.cross(&b);
  println!("{a} x {b} = {c}");
  println!("||{}|| = {}", a, a.norm());

  println!("dist({},{}) = {}", a, b, a.dist(&b));
}
