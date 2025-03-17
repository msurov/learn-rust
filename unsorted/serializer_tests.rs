// https://serde.rs/deserialize-struct.html

use serde_json;
use serde_binary;
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize, Deserializer};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

fn json_serialization() {
  let point = Point { x: 1, y: 2 };
  let serialized : String = serde_json::to_string(&point).unwrap();
  println!("serialized = {}", serialized);
  let deserialized: Point = serde_json::from_str(&serialized).unwrap();
  println!("deserialized = {}, {}", deserialized.x, deserialized.y);
  assert_eq!(point, deserialized);
}

fn binary_serialization() {
  let point = Point { x: 1, y: 2 };
  let serialized : Vec<u8> = serde_binary::to_vec(&point, serde_binary::binary_stream::Endian::Big).unwrap();
  println!("serialized = {:?}", serialized);
  let deserialized: Point = serde_binary::from_slice(&serialized, serde_binary::binary_stream::Endian::Big).unwrap();
  println!("deserialized = {}, {}", deserialized.x, deserialized.y);
  assert_eq!(point, deserialized);
}

#[derive(PartialEq, Debug)]
struct Data {
  time : f64,
  position : [f64; 3],
  velocity : [f64; 3],
}

impl Serialize for Data {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer
  {
    let mut s = serializer.serialize_struct("data", 3)?;
    s.serialize_field("time", &self.time)?;
    s.serialize_field("position", &self.position)?;
    s.serialize_field("velocity", &self.velocity)?;
    s.end()
  }
}

struct DataVisitor;

impl<'a> Visitor<'a> for DataVisitor {
  type Value = Data;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Result::Ok(())
  }

  fn visit_map<A>(self, mut map : A) -> Result<Self::Value, A::Error>
    where
      A : serde::de::MapAccess<'a>, 
  {
    let mut data = Data {time : 0., position : [0.; 3], velocity : [0.; 3]};

    let entry = map.next_entry::<String,f64>().expect("Can't extract the first field");
    let entry = entry.expect("Can't parse the first field");
    data.time = entry.1;

    let entry = map.next_entry::<String,[f64;3]>().expect("Can't extract the second field");
    let entry = entry.expect("Can't parse the second field");
    data.position = entry.1;

    let entry = map.next_entry::<String,[f64;3]>().expect("Can't extract the third field");
    let entry = entry.expect("Can't parse the third field");
    data.velocity = entry.1;

    Ok(data)
  }
}

impl<'de> Deserialize<'de> for Data {
  fn deserialize<D>(deserializer: D) -> Result<Data, D::Error>
  where
    D : Deserializer<'de>,
  {
    let fields : &'static [&str] = &[
      "time",
      "position",
      "velocity",
    ];
    let data = deserializer.deserialize_struct("data", fields, DataVisitor)?;
    Ok(data)
  }
}

fn data_custom_serialization() {
  let data = Data { 
    time : 1.0,
    position : [1., 2., -3.],
    velocity : [0., 7., 12.]
  };

  let serialized : Vec<u8> = serde_binary::to_vec(&data, serde_binary::binary_stream::Endian::Big).unwrap();
  println!("serialized = {:?}", serialized);
  let deserialized: Data = serde_binary::from_slice(&serialized, serde_binary::binary_stream::Endian::Big).unwrap();
  println!("deserialized = [{}, {:?}, {:?}]", deserialized.time, deserialized.position, deserialized.velocity);
  assert_eq!(data, deserialized);
}

fn main() {
  data_custom_serialization();
  binary_serialization();
  json_serialization();
}
