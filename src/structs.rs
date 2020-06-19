

struct Car{
    engine: String,
    name: String,
    cc: i32,
}

impl Car {
    fn new(name:String, engine: String, cc: i32) -> Car{
        Car {
            name,
            engine,
            cc,
        }
    }

    fn get_fullname(&self) {
        println!("Car Name: {} {} {}", self.name, self.engine, self.cc)
    }
}

fn main() {
    let my_car: Car = Car::new("Audi".to_string(),"BS4".to_string(),1000);
    my_car.get_fullname();
}
