// https://crates.io/crates/packed_struct/0.10.1
use packed_struct::prelude::*;


#[derive(PackedStruct, PartialEq, Debug)]
#[packed_struct(bit_numbering="msb0")]
pub struct TestPack {
    #[packed_field(bits="0..=2")]
    tiny_int: Integer<u8, packed_bits::Bits::<3>>,
    #[packed_field(bits="3..=4", ty="enum")]
    mode: SelfTestMode,
    #[packed_field(bits="7")]
    enabled: bool
}

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum SelfTestMode {
    NormalMode = 0,
    PositiveSignSelfTest = 1,
    NegativeSignSelfTest = 2,
    DebugMode = 3,
}

fn main() {
  let test = TestPack {
    tiny_int: 5.into(),
    mode: SelfTestMode::DebugMode,
    enabled: true
  };

  // pack into a byte array
  let packed: [u8; 1] = test.pack().unwrap();
  assert_eq!([0b10111001], packed);

  // unpack from a byte array
  let unpacked = TestPack::unpack(&packed).unwrap();
  assert_eq!(*unpacked.tiny_int, 5);
  assert_eq!(unpacked.mode, SelfTestMode::DebugMode);
  assert_eq!(unpacked.enabled, true);

  // or unpack from a slice
  let unpacked = TestPack::unpack_from_slice(&packed[..]).unwrap();
  assert_eq!(unpacked, test);
}
