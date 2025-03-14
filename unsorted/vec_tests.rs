

fn filter_even(numbers: Vec<i32>) -> Vec<i32> {
  let mut result = Vec::new();
  for e in &numbers {
    if e % 2 == 0 {
      result.push(*e);
    }
  }
  return result;
}

fn reverse_vector(numbers: Vec<i32>) -> Vec<i32> {
  let n = numbers.len();
  let mut result = vec![0i32; n];
  for i in 0..n {
    result[n - i - 1] = numbers[i];
  }
  return result;
}

fn main() {
  let mut nitems = String::new();
  std::io::stdin().read_line(&mut nitems).expect("Can't read number of items");
  let nitems = nitems.trim().parse::<u32>().expect("Can't parse number of items");

  let mut items : Vec<String> = Vec::new();
  items.reserve(nitems as usize);

  for _ in 1..=nitems {
    let mut item = String::new();
    std::io::stdin().read_line(&mut item).expect("Can't read item name");
    items.push(
      item.trim().to_owned()  
    );
  }

  println!("{:?}", items);
}

#[allow(unused)]
fn tests() {
  let arr1 = Vec::<i32>::new();

  let arr2: Vec<f64> = Vec::new();
  let arr2_moved_here = arr2;
  // arr2 is invalid anymore, because arr2_moved_here owns its data

  let arr3 = vec![1, 2, 3, 4, 5];
  assert!(arr3.len() == 5);
  let arr4 = vec![0.0f64; 100]; // vec of 100 elements filled with 0
  assert!(arr4.capacity() >= 100);

  {
    let mut arr = vec![1, 2, 3];
    arr.push(4);
    arr.push(5);
    arr.insert(0, 10);
    println!("{:?}", arr);
  }

  {
    let mut arr = Vec::new(); // empty array of any type
    arr.push("abc");
    arr.push("def");
    arr.insert(0, "xyz");
    println!("{:?}", arr);
  }

  {
    let arr = vec![1.0, 2.0, 3.0];
    println!("arr[0] = {}", arr[0]);
    println!("arr[1] = {}", arr[1]);
    println!("arr[2] = {}", arr[2]);

    // get elem with option monad and match it
    if let Some(elem) = arr.get(2) {
      println!("elem at index {} equals {}", 2, elem);
    }
    if let None = arr.get(10) {
      println!("there is no elem at index 10");
    }
  }

  {
    let arr = [1,2,3,4,5];

    let mut i = 10;
    if let Some(elem) = arr.get(i) {
      println!("elem at index {} equals {}", i, elem);
    } else {
      println!("there is no elem at index {}", i);
    }

    i = 3;
    if let Some(elem) = arr.get(i) {
      println!("elem at index {} equals {}", i, elem);
    } else {
      println!("there is no elem at index {}", i);
    }
  }

  {
    let arr = vec![1,2,3,4,5];
    for e in &arr { // we must use &arr here, otherwise the for loop takes ownership
      print!("{} ", *e);
    }
    assert_eq!(arr[0], 1);
    println!();
  }

  {
    let mut arr = vec![1,2,3,4,6,5];
    let removed = arr.remove(4);
    assert_eq!(arr[4], 5);
    assert_eq!(removed, 6);
  }

  {
    let mut arr = vec!["a", "b", "c"];
    // this will print: c b a,
    //  and will clear the array
    while let Some(elem) = arr.pop() {
      print!("{} ", elem);
    }
    println!("");
  }
}
