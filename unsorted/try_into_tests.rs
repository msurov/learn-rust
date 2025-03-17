// https://stackoverflow.com/questions/66042585/rust-can-i-have-a-fixed-size-slice-by-borrowing-the-whole-fixed-size-array-in-a

use sub_array::{self, SubArray};

fn main() {
  let a = [1,2,3,4,5,6,7,8];
  // option 1: resulting array type differs
  let sub_a : &[i32] = &a[2..6];
  let sub_a : &[i32; 4] = a[2..6].try_into().unwrap();
  // option 2: using slice patterns
  let [_, _, ref sub_a @ .., _, _] = a;
  // option 3: using the 3-rd party library sub_array
  let sub_a : &[i32; 4] = a.sub_array_ref(2);
  // option 4: using unsafe block
  let sub_a = unsafe {
    let ptr : *const i32 = a[2..6].as_ptr();
    &(*(ptr as *const [i32; 4]))
  };
}
