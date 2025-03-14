/** 
 * Traits:
 * ------
 *  ToString::to_string(): converts a (i32, &str, f32, etc) value to String representation
 *  Clone::clone(): makes deep copy of an object
 *  ToOwnder::to_owned(): borrowing -> owning copy; type of owning copy may differ from the source object type
 *                        example to_owned(&str) -> String
 **/

 #[allow(dead_code)]
fn const_ref_example() {
  let x : i32 = 5;
  let px : &i32 = &x;
  println!("x = {}", *px);
}

#[allow(dead_code)]
fn mut_ref_example() {
  let mut x : i32 = 5;
  let px : &mut i32 = &mut x;
  *px = 8;
  println!("x = {}", x);
}

#[allow(dead_code)]
fn multiple_refs_example() {
  let mut x : i32 = 5;
  let mut_ref_x = &mut x;
  *mut_ref_x = 8;
  let im_ref_x = &x;
  println!("x = {}", *im_ref_x);
}

#[allow(dead_code)]
fn bind_example() {
  let bind = String::from("hi2");
  println!("bind = {}", bind);
}

#[allow(dead_code)]
fn reborrow_example() {
  let mut a = 7;
  let b = &mut a;
  // let c = &mut a; // this would fail, because there are two mutable references on the same variable
  let c = &mut *b; // this is "reborrowing the reference"
  println!("{}", c);
}

/**
 * a function car return a reference on a static variable 
 * or on an argument only
 */
fn get_str_ref() -> &'static str {
  static A : &str = "xyz";
  return &A;
}

fn static_ref_example() {
  let a = get_str_ref();
  println!("{:p}: {}", a, a);
}

#[allow(dead_code)]
/**
 * we say, that the result reference has the same live-time as the inputs
 */
fn max<'inp>(a : &'inp i32, b : &'inp i32) -> &'inp i32
{
  if *a > *b {
    return a;
  } else {
    return b;
  }
}

#[allow(dead_code)]
fn use_annotations_example() {
  let a = 3;
  let b = 5;
  let maxval = max(&a, &b);
  println!("{maxval}");
}

#[allow(dead_code)]
fn prolongate_temporarily_value() {
  let temp_val = &String::from("abc");
  println!("{temp_val}");
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

#[allow(dead_code)]
fn strings_refs() {
  let s1: &str = "aaabbbccc";
  println!("{:p}", s1.as_ptr());
}

#[allow(dead_code)]
fn clone_val() {
  let original_val = &1;
  let cloned_val : i32 = Clone::clone(original_val);

  println!("orig ptr:   {:p}", original_val);
  println!("cloned ptr: {:p}", &cloned_val);
}

#[allow(dead_code)]
fn clone_ref() {
  let name = "abc".to_owned();
  let name_clone = (&name).clone(); // Явно берем ссылку и вызываем .clone()
  println!("{:p}", name.as_ptr());
  println!("{:p}", name_clone.as_ptr());
}

#[allow(dead_code)]
fn move_owning() {
  // these simple values are just copied when assigning,
  // because they implement the trait Copy
  let a = 1;
  let b = a;
  let c = b;
  println!("{c}");
  println!("{b}");
  println!("{a}");

  // shallow copy works here: the pointer to static string and size are being copied
  let q1 = "abc";
  let q2 = q1;
  let q3 = q2;
  println!("{q1}");
  println!("{q2}");
  println!("{q3}");

  // But the string does not implement the trait Copy, then
  // when assigning we change the ownership
  // it works like std::move in C++
  let s1 = "abc".to_string();
  let s2 = s1;
  let s3 = s2;

  // now we can acceess s3, but s1 and s2 are not valid anymore
  println!("{s3}");
  // the next code failes, because s3 ownes the original object now:
  //    println!("{s2}");
  //    println!("{s3}");
}

#[allow(dead_code)]
fn clone_examples() {
  // &str is like string range
  // when clonning &str, the pointer and string length are being copied
  let q1 : &str = "abc";
  let q2 : &str = Clone::clone(&q1);
  assert_eq!(q1.as_ptr(), q2.as_ptr());

  // String allocates memory
  // when clonning String object, 
  // the process allocates new memory and copies all the data
  let s1 : String = "abc".to_string();
  let s2 : String = Clone::clone(&s1);
  let s3 : String = s2.clone();
  let s4 : String = (&s2).clone();

  assert_ne!(s1.as_ptr(), s2.as_ptr());
  assert_ne!(s2.as_ptr(), s3.as_ptr());
  assert_ne!(s3.as_ptr(), s4.as_ptr());
}

fn static_str_example() {
  let s1 = "abc";
  let s2 = Clone::clone(&s1);
  println!("{:p}", s1.as_ptr());
  println!("{:p}", s2.as_ptr());
}

fn clamp(a : f32, from : f32, to : f32) -> f32 {
  return if a < from { from }
    else if a > to { to }
    else { a };
}

fn clamp_example() {
  let a = 7.0f32;
  let clamped_a = clamp(a, -5.0f32, 5.0f32);
  println!("clamped {a} = {clamped_a}");
}

fn let_if_example() {
  let condition= false;
  let number = if condition { 4 } else { 5 };
  println!("{}", number);
}

fn ref_definitions() {
  let a = 30;
  let ref p1 = a;
  let p2 = &a;
  assert_eq!(p1, p2);
  let b = p2 + 1; // is value, dereferencing is applied automatically
  assert_eq!(b, a + 1);
}

fn modify_tuple_component_by_ref() {
  let mut a = (3, 4, 5);
  if let (3, ref mut a1, _) = a {
    *a1 = *a1 + 1;
  }
  assert_eq!(a, (3, 5, 5));
}

fn main() {
  const_ref_example();
  mut_ref_example();
  multiple_refs_example();
  bind_example();
  reborrow_example();
  use_annotations_example();
  prolongate_temporarily_value();
  strings_refs();
  clone_val();
  clone_ref();
  move_owning();
  clone_examples();
  static_str_example();
  static_ref_example();
  let_if_example();
  clamp_example();
  ref_definitions();
  modify_tuple_component_by_ref();
}
