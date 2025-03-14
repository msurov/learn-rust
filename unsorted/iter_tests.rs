
#[allow(unused)]
fn array_iterator_test() {
  let arr = [1,2,3];
  let mut arr_it = arr.iter();
  let arr_1st = arr_it.next();
  let arr_2nd = arr_it.next();
  let arr_3rd = arr_it.next();
  let arr_4th = arr_it.next();
  assert_eq!(arr_4th, None);

  println!("array elems: {:?}, {:?}, {:?}, {:?}", arr_1st, arr_2nd, arr_3rd, arr_4th);
}

fn gen_fib_seq(nelems : usize) -> Vec<u32> {
  let mut result = Vec::new();
  result.resize(nelems, 0u32);

  let mut it = result.iter_mut();
  let mut prev : u32 = 1;
  let mut cur : u32 = 0;

  loop {
    if let Some(e) = it.next() {
      *e = cur;
      let next = prev + cur;
      prev = cur;
      cur = next;
    } else {
      break;
    }
  }

  return result;
}

#[allow(unused)]
fn array_mut_iterator_test() {
  let fib = gen_fib_seq(40);
  println!("fib seq: {:?}", fib);
  assert_eq!(fib[2], 1);
  assert_eq!(fib[10], 55);
}

#[allow(unused)]
fn owning_iter_test() {
  {
    let arr = ["a".to_owned(), "b".to_owned(), "c".to_owned()];
    let it = arr.into_iter(); // here it owns arr
    for e in it {
      print!("{}", e);
    }
    println!();
    // into_iter owns arr, so arr is not accessible anymore
    // error[E0382]: borrow of moved value: `arr`
    // println!("{:?}", arr);
  }

  {
    let arr = [1, 2, 3];
    let it = arr.into_iter(); // here it owns a copy of arr
    for e in it {
      print!("{}", e);
    }
    println!();
    // arr is still alive
    println!("{:?}", arr); // ok
  }

  {
    let arr = vec![1,2,3,4,5];
    let it = arr.into_iter(); // here it owns arr
    for e in it {
      print!("{}", e);
    }
    println!();
    // into_iter owns arr, so arr is not accessible anymore
    // error[E0382]: borrow of moved value: `arr`
    // println!("{:?}", arr);
  }
}

#[allow(unused)]
fn iterator_to_collection() {
  let arr = [1,2,3,4,5];
  let vec : Vec<i32> = arr.into_iter().collect();
}

#[allow(unused)]
fn map_array() {
  {
    let natural_numbers = 1..;
    let mut odd_numbers = natural_numbers.into_iter().filter(| x | *x % 2 == 0);
    assert_eq!(odd_numbers.next().unwrap(), 2);
    assert_eq!(odd_numbers.next().unwrap(), 4);
    assert_eq!(odd_numbers.next().unwrap(), 6);
    assert_eq!(odd_numbers.next().unwrap(), 8);
  }

  {
    let natural_numbers = 1..;
    let mut squares = natural_numbers.into_iter().map(| x | x * x);
    assert_eq!(squares.next().unwrap(), 1);
    assert_eq!(squares.next().unwrap(), 4);
    assert_eq!(squares.next().unwrap(), 9);
    assert_eq!(squares.next().unwrap(), 16);
  }

  {
    let seq = 1..=5;
    let sum : i32 = seq.clone().into_iter().sum();
    println!("Sum of the sequence ({:?}) is {}", seq, sum);
  }

  {
    let seq = 1..=10;
    let x : i32 = seq.into_iter()
      .filter(| x | *x % 3 == 0)
      .map(| x | x * x)
      .sum();
    println!("x = {}", x);
  }

  {
    let arr = [1,2,3,4,5];
    // here, arr.into_iter is being called
    // so the array will be copied
    for e in arr {
      print!("{e} ");
    }
    println!();
    // to avoid copying, one needs to use ref
    for e in &arr {
      print!("{e} ");
    }
    println!();
    // or explicitly specify
    for e in arr.iter() {
      print!("{e} ");
    }
    println!();
  }
}

fn ranges_test() {
  let r = 1..;
  let q = r.map_while(|x| if x < 10 { Some(x) } else { None::<i32> });
  let arr : Vec<i32> = q.collect();
  println!("{:?}", arr);
}

fn is_prime(n : u32) -> bool {
  let mut k = 2;
  loop {
    if k * k > n {
      return true;
    }
    if n % k == 0 {
      return false;
    }
    k += 1;
  }
}

fn get_primes_generator() -> impl Iterator<Item = u32> {
  let r = 2u32..;
  let f = r.filter(|px| is_prime(*px));
  return f;
}

fn test_primes() {
  let mut primes = get_primes_generator();
  let val = primes.nth(999);
  println!("999-th prime is {}", val.unwrap());
}

fn main() {
  array_iterator_test();
  array_mut_iterator_test();
  owning_iter_test();
  map_array();
  ranges_test();
  test_primes();
}
