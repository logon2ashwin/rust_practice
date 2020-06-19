// use std::io;
//
// pub fn generate_fibonacci_from_index() {
//     let mut index = 0;
//     let mut fobbo_index = String::new();
//
//     println!("Enter the fibonacci index");
//     io::stdin()
//         .read_line(&mut fobbo_index)
//         .expect("Failed to read line");
//
//     let fibbo_index: i32 = match fibbo_index.trim().parse() {
//         Ok(num) => num
//         Err(_) => {
//             println!("Enter a valid number!");
//             continue;
//         },
//     };
//
//     let mut fibbo_array = [0,fibbo_index];
//
//     for i in fibbo_array.iter() {
//         println!("current value {}", i);
//     }
// }
