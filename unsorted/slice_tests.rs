
#[allow(unused)]
fn get_first_half(arr : &[i32]) -> &[i32] {
  let n = arr.len();
  return &arr[..n/2];
}

#[allow(unused)]
fn fill_first_half(arr : &mut [u8], value : u8) {
  let n = arr.len();
  let first_half = &mut arr[..n/2];
  first_half.fill(value);
}

#[allow(unused)]
fn mut_slices_test() {
  let mut arr = vec![0u8; 100];
  fill_first_half(&mut arr, 128);

  let diap = &mut arr[20..30];
  diap.fill(255);

  let first10 = &mut arr[..10];
  first10.fill(5);

  let tail = &mut arr[90..];
  tail.fill(30);

  println!("{:?}", arr);
}

#[allow(unused)]
fn slices_test() {
  let natural_numbers = 1..1000;
  let mapped = natural_numbers.filter(|x| *x % 3 == 0).map(|x| x*x*x);
  let arr : Vec<i32> = mapped.collect();
  let sub_arr : &[i32] = &arr[10..100];
  println!("{:?}", sub_arr);
}

#[allow(unused)]
fn static_str_test() {
  let s : &'static str = "hello";
  println!("{}", s);
}

#[allow(unused)]
fn string_chars_iterator() {
  let s = "Привет 😀";
  let mut iter = s.chars();
  for e in iter {
    print!("{} ", e);
  }
  println!();
}

fn split_terator() {
  let s = "abc def xyz";
  let mut substrings_iter = s.split_whitespace();
  assert_eq!(substrings_iter.next().unwrap(), "abc");
  assert_eq!(substrings_iter.next().unwrap(), "def");
  assert_eq!(substrings_iter.next().unwrap(), "xyz");
  assert_eq!(substrings_iter.next(), None::<&str>);

  let s = "abc|def|xyz";
  let mut substrings_iter = s.split("|");
  assert_eq!(substrings_iter.next().unwrap(), "abc");
  assert_eq!(substrings_iter.next().unwrap(), "def");
  assert_eq!(substrings_iter.next().unwrap(), "xyz");
  assert_eq!(substrings_iter.next(), None::<&str>);

  let s = "привет";
  assert_eq!(s.bytes().count(), 12);
  assert_eq!(s.chars().count(), 6);

  let s = "hello\nworld";
  assert_eq!(s.lines().count(), 2);
}

fn main() {
  mut_slices_test();
  static_str_test();
  string_chars_iterator();
  split_terator();
}
