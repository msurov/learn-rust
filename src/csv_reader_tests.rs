#![allow(unused)]
mod csv_reader;
mod quat;
use csv_reader::CSVReader;
use quat::Quat;


fn main() {
  let mut reader = CSVReader::new("data/quats.csv");
  let mut arr = Vec::<Quat<f64>>::new();
  while let Some(line) = reader.take() {
    arr.push(Quat::from(line));
  }
  let mut q = Quat::from(1.0f64);
  for e in &arr {
    q = (q / (*e)).normalized();
  }
  println!("{q}");
}
