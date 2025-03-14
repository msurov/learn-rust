use std::time::{Duration, Instant};

fn main() {
  let t1 = Instant::now();
  let t2 = Instant::now();
  println!("{:?}", t2 - t1);
}
