extern crate nalgebra as na;
use na::{Matrix2x3, Vector2, RowVector3};

fn main() {
  let m1 = Matrix2x3::new(1.1, 1.2, 1.3,
                        2.1, 2.2, 2.3);

  let m2 = Matrix2x3::from_rows(&[
      RowVector3::new(1.1, 1.2, 1.3),
      RowVector3::new(2.1, 2.2, 2.3)
  ]);

  let m3 = Matrix2x3::from_columns(&[
      Vector2::new(1.1, 2.1),
      Vector2::new(1.2, 2.2),
      Vector2::new(1.3, 2.3)
  ]);

  println!("m1 = {}", m1);
  println!("m2 = {}", m2);
  println!("m3 = {}", m3);
}