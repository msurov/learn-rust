
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
struct ABC {
  a : f32,
  b : [f32; 3],
  c : [f32; 3]
}

impl From<ABC> for f32 {
  fn from(value: ABC) -> Self {
    value.a
  }
}

fn main() {
  let x = ABC {a : 55., b : [1., 2., 3.], c : [1., 2., 3.]};
  let y : f32 = x.into();
  println!("y = {y}");
}
