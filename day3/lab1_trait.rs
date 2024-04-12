// Create a trait called Vehicle
trait Vehicle {
    fn speed(&self) -> f64;
    fn display(&self);
}

struct Car {
    speed: f64,
}

struct Bicycle {
    speed: f64,
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        return self.speed;
    }

    fn display(&self) {
        println!("The speed of the car is {}", self.speed);
    }
}

impl Vehicle for Bicycle {
    fn speed(&self) -> f64 {
        return self.speed;
    }

    fn display(&self) {
        println!("The speed of the bicycle is {}", self.speed);
    }
}

fn main() {
    let car = Car { speed: 12.0 };
    let bicycle = Bicycle { speed: 15.0 };
    car.display();
    bicycle.display();
}
