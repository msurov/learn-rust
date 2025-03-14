#![allow(dead_code)]

use num::{Float, Num, Signed};
use std::{fmt::Display, ops::{Add, Sub, Div, Mul, Neg}};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quat<T> where T : Num + Signed + Copy {
  pub w : T,
  pub x : T,
  pub y : T,
  pub z : T
}

impl<T> Quat<T> where T : Num + Signed + Copy {
  fn elementwise_op(&self, other : &Self, op : fn(&T, &T) -> T) -> Self {
    Quat {
      w : op(&self.w, &other.w),
      x : op(&self.x, &other.x),
      y : op(&self.y, &other.y),
      z : op(&self.z, &other.z)
    }
  }

  pub fn make_scal(w : T) -> Quat<T> {
    let zero = T::zero();
    Quat {
      w, x : zero, y : zero, z : zero
    }
  }

  pub fn make_vec(x : T, y : T, z : T) -> Quat<T> {
    let zero = T::zero();
    Quat {
      w : zero, x, y, z
    }
  }

  pub fn vec(&self) -> [T; 3] {
    [self.x, self.y, self.z]
  }

  pub fn add_quat(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a : &T, b : &T| (*a) + (*b))
  }

  pub fn sub_quat(&self, other : &Self) -> Self {
    self.elementwise_op(other, |a : &T, b : &T| (*a) - (*b))
  }

  pub fn conj(&self) -> Self {
    Quat {
      w : self.w,
      x : -self.x,
      y : -self.y,
      z : -self.z
    }
  }

  pub fn sq(&self) -> T {
    self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn mul_quat(&self, other : &Self) -> Self {
    Quat {
      w : self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
      x : self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
      y : self.w * other.y + self.y * other.w - self.x * other.z + self.z * other.x,
      z : self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x,
    }
  }

  pub fn mul_conj(&self, other : &Self) -> Self {
    Quat {
      w :  self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z,
      x : -self.w * other.x + self.x * other.w - self.y * other.z + self.z * other.y,
      y : -self.w * other.y + self.y * other.w + self.x * other.z - self.z * other.x,
      z : -self.w * other.z + self.z * other.w - self.x * other.y + self.y * other.x,
    }
  }

  pub fn inv(&self) -> Self {
    let den = self.sq();
    self.conj().div_num(den)
  }

  pub fn mul_num(&self, k : T) -> Self {
    Quat {
      w : self.w * k,
      x : self.x * k,
      y : self.y * k,
      z : self.z * k,
    }
  }

  pub fn div_num(&self, k : T) -> Self {
    Quat {
      w : self.w / k,
      x : self.x / k,
      y : self.y / k,
      z : self.z / k,
    }
  }

  pub fn div_quat(&self, other : &Self) -> Self {
    self.mul_conj(other).div_num(other.sq())
  }

}

impl<T> Quat<T> where T : Float + Signed + Copy {
  pub fn norm(&self) -> T {
    T::sqrt(self.sq())
  }

  pub fn normalized(&self) -> Self {
    self.div_num(self.norm())
  }
}

impl<T> From<&Vec<T>> for Quat<T> where T : Num + Copy + Signed {
  fn from(arr: &Vec<T>) -> Self {
    assert_eq!(arr.len(), 4);
    Quat {
      w : arr[0],
      x : arr[1],
      y : arr[2],
      z : arr[3]
    }
  }
}

impl<T> From<T> for Quat<T> where T : Num + Copy + Signed {
  fn from(val : T) -> Self {
    let zero = T::zero();
    Quat::make_scal(val)
  }
}

impl<T> Neg for Quat<T> where T : Num + Signed + Copy {
  type Output = Quat<T>;

  fn neg(self) -> Self::Output {
    Quat {
      w : -self.w,
      x : -self.x,
      y : -self.y,
      z : -self.z,
    }
  }
}

impl<T> Add for Quat<T> where T : Num + Signed + Copy {
  type Output = Quat<T>;

  fn add(self, other: Self) -> Self::Output {
    self.add_quat(&other)
  }
}

impl<T> Sub for Quat<T> where T : Num + Signed + Copy {
  type Output = Quat<T>;

  fn sub(self, other: Self) -> Self::Output {
    self.sub_quat(&other)
  }
}

impl<T> Mul for Quat<T> where T : Num + Signed + Copy {
  type Output = Quat<T>;

  fn mul(self, other : Self) -> Self::Output {
    self.mul_quat(&other)
  }
}

impl<T> Div for Quat<T> where T : Num + Signed + Copy {
  type Output = Quat<T>;

  fn div(self, other: Self) -> Self::Output {
    self.div_quat(&other)
  }
}

impl<T> Display for Quat<T> where T : Num + Signed + Copy + Display {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {}, {})", self.w, self.x, self.y, self.z)
  }
}
