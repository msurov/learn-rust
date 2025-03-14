mod csv_reader;
mod quat;
use std::time::Instant;
use csv_reader::CSVReader;
use quat::Quat;

fn load_quats_from_csv(filepath : &str) -> Vec::<Quat<f64>> {
  let mut reader = CSVReader::new(filepath);
  let mut arr = Vec::new();
  while let Some(line) = reader.take() {
    let q = Quat {
      w : line[0] as f64,
      x : line[1] as f64,
      y : line[2] as f64,
      z : line[3] as f64
    };
    arr.push(q);
  }
  arr
}

fn main() {
  let arr = load_quats_from_csv("data/quats.csv");

  let t1 = Instant::now();
  let mut q = Quat::make_scal(1.0f64);
  for e in &arr {
    q = (q / (*e)).normalized();
  }
  let t2 = Instant::now();
  println!("elapsed in {:?}", t2 - t1);
  println!("{q}");
}
