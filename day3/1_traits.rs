trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return 3.14159265 * self.radius * self.radius;
    }
}

fn main() {
    let rect = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    let circle = Circle { radius: 10.0 };
    println!("The area of the rectangle is {}", rect.area());
    println!("The area of the circle is {}", circle.area());
}
