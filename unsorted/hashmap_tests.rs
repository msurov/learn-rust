use std::collections::HashMap;
use std::io;


#[allow(unused)]
fn create_hashmap_test() {
  let mut m : HashMap<&str,i32> = HashMap::new();
  m.insert("max", 3);
  m.insert("elena", 5);
  println!("{:?}", m);

  let m = HashMap::from([
    ("max", 3),
    ("elena", 5)
  ]);
  println!("{:?}", m);

  let arr = [
    (1, 1),
    (2, 4),
    (3, 9),
    (4, 16),
  ];
  let m1 : HashMap<_,_> = arr.into_iter().collect();
  let m2 : HashMap<_,_> = arr.iter().cloned().collect();
  let m3 : HashMap<_,_> = arr.iter().map(|x| *x).collect();

  println!("m = {:?}", m1);
  println!("m = {:?}", m2);
  println!("m = {:?}", m3);
}

#[allow(unused)]
fn access_hashmap_test() {
  let mut m = HashMap::from([
    ("Max", 92),
    ("Elena", 5),
    ("Oleg", 7)
  ]);
  if let Some(value) = m.get("Max") {
    println!("Max is {}", value);
  }
  if let Some(value) = m.get("Nick") {
    println!("Nick is {}", value);
  }

  for (key, value) in &m {
    println!("{} - {}", *key, *value);
  }

  assert!(m.contains_key("Max"));
  m.insert("Max", 77);
  assert_eq!(m["Max"], 77);

  // apply modification for selected element if it exists
  m.entry("Elena").and_modify(|x| { *x += 1; });
  assert_eq!(m["Elena"], 6);

  // this is equaivalent to
  if let Some(value) = m.get_mut("Elena") {
    *value += 1;
  }
  assert_eq!(m["Elena"], 7);

  m.remove("Oleg");
  assert_eq!(m.get("Oleg"), None::<&i32>);
}


#[allow(unused)]
fn translator() {
  let dictionary: HashMap<&str, &str> = HashMap::from([
    ("яблоко", "apple"),
    ("дом", "house"),
    ("книга", "book")
  ]);

  let mut inp = String::new();
  io::stdin().read_line(&mut inp).expect("Ошибка при чтении строки");
  let inp_lower = inp.trim().to_lowercase();
  let inp = inp_lower.as_str();

  if let Some(translation) = dictionary.get(inp) {
    println!("Перевод: {translation}");
  } else {
    println!("{inp} нет в словаре");
  }
}

fn read_trophy() -> String {
  let mut inp = String::new();
  std::io::stdin().read_line(&mut inp).expect("Can't read trophy");
  return inp.trim().to_owned();
}

#[allow(unused)]
fn trophy_test() {

  let mut num_trophies = String::new();
  io::stdin().read_line(&mut num_trophies).expect("Не удалось прочитать количество трофеев");
  let num_trophies: u32 = num_trophies.trim().parse().expect("Пожалуйста, введите корректное число");

  let mut trophies = HashMap::<String, u32>::new();

  for _ in 0..num_trophies {
    let trophy = read_trophy();
    if let Some(value) = trophies.get_mut(&trophy) {
      *value += 1;
    } else {
      trophies.insert(trophy, 1);
    }
  }

  for (trophy, nitems) in &trophies {
    println!("{}: {}", trophy, nitems);
  }
}

fn main() {
  access_hashmap_test();
  trophy_test();
}
