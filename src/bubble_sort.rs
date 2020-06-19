

pub fn bubble_sort(src_arr: &mut [i32]) {
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
    println!("final array : {:?}", src_arr);
}

fn main() {
    let mut arr: [i32; 10] = [6,4,3,7,8,1,2,3,0,9];
    bubble_sort(&mut arr);

}
