// Celsius to Farenheit

fn main() {
    println!("Enter the Celsius value");


    loop {
        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: i32 = match celsius.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Enter a valid number!, Entered value is {}", celsius);
                continue;
            },
        };

        println!("Value of {} celsius is {:.1}", celsius, (celsius as f64 * (1.8) as f64 + 32.00));
    }
}
