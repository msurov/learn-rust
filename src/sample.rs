use std::{fs, time::{Duration, Instant}};

fn main() {
  let dir = fs::read_dir("/").unwrap();
  for e in dir {
    println!("{}", e.unwrap().path().display());
  }
}
