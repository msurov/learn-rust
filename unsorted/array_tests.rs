
#[allow(unused)]
fn copy_array_test() {
  let a = [0u32; 10];
  let copy_of_a = a;
  assert_ne!(a.as_ptr(), copy_of_a.as_ptr());
  // the both arrays are alive
  println!("a = {:?}", a);
  println!("copy of a = {:?}", copy_of_a);

  let b = ["a".to_owned(), "b".to_owned(), "c".to_owned()];
  let copy_of_b = b.clone();
  let moved_own_of_b = b;

  // b is not accessible anymore, the array is not copyable, so move works
  // because array element is not copyable
  println!("{:?}", moved_own_of_b);
  println!("{:?}", copy_of_b);
}

#[allow(unused)]
fn unpack_array() {
  let mut a : [u16; 3] = [0u16; 3];

  if let [0, ref mut a1, _] = a {
    *a1 += 1;
  }

  println!("{:?}", a);
}

#[allow(unused)]
fn filling_arrays() {
  let mut numbers = [0u32; 10]; // array of 10 zeros of type u32

  for i in 1..=numbers.len() {
    numbers[i - 1] = i as u32;
  }

  let mut sum : u32 = 0;

  for elem in &numbers {
    sum += *elem;
  }

  print!("array {:?}", numbers);
  println!(", sum of their elements is {sum}");
}

#[allow(unused)]
fn declare() {
  let a : [f32; 10];
  a = [1.0f32; 10];
  println!("array {:?}", a);
}

#[allow(unused)]
fn sum_even_numbers(numbers: &[i32]) -> i32 {
  let mut sum : i32 = 0;

  for e in numbers {
    if e % 2 == 0 { 
      sum += e;
    }
  }

  return sum;
}

#[allow(unused)]
fn friends_list_test() {
  let friends_list: [&str; 4] = ["Алиса", "Борис", "Виктория", "Григорий"];

  let mut friend_no = String::new();
  std::io::stdin().read_line(&mut friend_no).expect("Ошибка чтения ввода");
  let friend_no : usize = friend_no.trim().parse().expect("Пожалуйста, введите корректное число.");

  if friend_no < 1 || friend_no > friends_list.len() {
    println!("Некорректный номер друга.");
  } else {
    println!("Друг номер {}: {}", friend_no, friends_list[friend_no - 1])
  }
}

#[allow(unused)]
fn test_grades() {
  let grades: [u8; 20] = [5, 4, 3, 5, 2, 4, 5, 3, 4, 5, 5, 4, 3, 2, 5, 4, 5, 5, 4, 3];
  let mut hist: [u32; 5] = [0u32; 5];

  for grade in grades {
    if grade < 1 || grade > 5 {
      panic!("incorrect grade");
    }
    let i = (grade - 1) as usize;
    hist[i] += 1;
  }

  println!("Отличников: {}", hist[4]);
  println!("Хорошистов: {}", hist[3]);
  println!("Троечников: {}", hist[2]);
  println!("Двоечников: {}", hist[1]);
}

#[allow(dead_code)]
fn compare_items_test() {
  const NITEMS : usize = 5;
  let mut ordered_items : [String; NITEMS] = [String::new(), String::new(), String::new(), String::new(), String::new()];
  let mut delivered_items : [String; NITEMS] = [String::new(), String::new(), String::new(), String::new(), String::new()];

  for i in 0..NITEMS {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("Can't read ordered item");
    ordered_items[i] = inp.trim().to_owned();
  }

  for i in 0..NITEMS {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).expect("Can't read delivered item");
    delivered_items[i] = inp.trim().to_owned();
  }

  let mut nincorrect = 0;

  for i in 0..NITEMS {
    if ordered_items[i] != delivered_items[i] {
      nincorrect += 1;
      println!("{}", i + 1);
    }
  }

  if nincorrect == 0 {
    println!("0");
  }

}

#[allow(unused)]
fn swap_array_elems() {
  let mut arr = [1,2,3,4,5,];
  // let p0 = &mut arr[0];
  // let p4 = &mut arr[4];
  // std::mem::swap(p0, p4);
  println!("{:?}", arr);
  let (p0, p4) = arr.split_at_mut(4);
  std::mem::swap(&mut p0[0], &mut p4[0]);
  println!("{:?}", arr);
}

fn main() {
  filling_arrays();
  copy_array_test();
  unpack_array();
  declare();
  swap_array_elems();
}
