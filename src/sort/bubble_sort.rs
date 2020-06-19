pub fn run(src_arr: &mut [i32]) -> &mut [i32] {
  loop {
      let mut swapped  = false;
      for idx in 0..(src_arr.len()-1) {
          if src_arr[idx] > src_arr[idx+1] {
              src_arr.swap(idx, idx+1);
              swapped = true;
              continue;
          }
      }
      if swapped == false {
          break;
      }
  }
  src_arr
}