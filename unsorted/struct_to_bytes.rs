use std::ops::Add;

use sub_array::SubArray;


#[derive(Debug, PartialEq)]
struct Data {
  a : f64,
  p : [f64; 3],
  v : [f64; 3],
}

impl Data {
  const SIZE : usize = std::mem::size_of::<Data>();

  fn to_le_bytes(&self) -> [u8; Data::SIZE] {
    let mut mem = [0u8; Data::SIZE];

    let bytes = self.a.to_le_bytes();
    let mut i1 = 0 as usize;
    let mut i2 = bytes.len();
    mem[i1..i2].copy_from_slice(&bytes);

    for e in self.p {
      let bytes = e.to_le_bytes();
      i1 = i2;
      i2 = i2 + bytes.len();
      mem[i1..i2].copy_from_slice(&bytes);
    }

    for e in self.v {
      let bytes = e.to_le_bytes();
      i1 = i2;
      i2 = i2 + bytes.len();
      mem[i1..i2].copy_from_slice(&bytes);
    }

    mem
  }

  fn from_le_bytes(mem : [u8; Data::SIZE]) -> Data {
    const F64SIZE : usize = size_of::<f64>();
    let mut offset = 0;
    let mut data = Data {a : 0., p : [0.; 3], v : [0.; 3]};

    let elem : &[u8; F64SIZE] = mem.sub_array_ref(offset);
    data.a = f64::from_le_bytes(*elem);
    offset += F64SIZE;

    for e in &mut data.p {
      *e = f64::from_le_bytes(*mem.sub_array_ref(offset));
      offset += F64SIZE;
    }

    for e in &mut data.v {
      *e = f64::from_le_bytes(*mem.sub_array_ref(offset));
      offset += F64SIZE;
    }

    data
  }
}

fn test_serialization() {
  let data = Data {
    a : 1.0,
    p : [1., 2., 7.],
    v : [3., -1., 5.]
  };
  let serialized = data.to_le_bytes();
  println!("{:?}", serialized);

  let data2 = Data::from_le_bytes(serialized);
  assert_eq!(data, data2);
}

fn main() {

}
