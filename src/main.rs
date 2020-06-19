// use std::io;
use rust_practice::sort::bubble_sort;
//
//
// fn main() {
//     let mut current_index = 0;
//     let mut fibbo_index = String::new();
//
//     println!("Enter the fibbonacci index");
//     io::stdin()
//         .read_line(&mut fibbo_index)
//         .expect("Failed to read line");
//
//     let fibbo_index: u32 = fibbo_index.trim().parse().expect("Enter the valid input");
//
//     let mut fibbo_array = vec![0; fibbo_index as usize];
//
//     while current_index < fibbo_index as usize {
//         if current_index == 0 {
//             fibbo_array[current_index] = 0;
//         }
//         else if current_index == 1 {
//             fibbo_array[current_index] = 1;
//         }
//         else {
//             fibbo_array[current_index] = fibbo_array[current_index - 1] + fibbo_array[current_index - 2];
//         }
//         current_index += 1;
//     }
//
//     println!("The fibbonacci number at position {} is {}", fibbo_index, fibbo_array[(fibbo_index - 1) as usize]);
// }


fn main() {
    // let mut name: Option<String> = None;
    // let mut dummy = String::new();

    //     println!("Enter the fibbonacci index");
    //     io::stdin()
    //         .read_line(&mut dummy)
    //         .expect("Failed to read line");

    //     name = Some(dummy);
    //     match name{
    //         Some(val) => println!("Entered value is {}", val),
    //         None => println!("No value"),
    //     }
    let mut arr: [i32; 10] = [6,4,3,7,8,1,2,3,0,9];
    bubble_sort::run(&mut arr);
    println!("sorted array {:?}", arr);
}
