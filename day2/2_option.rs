fn main() {
    println!("Result is {:?}", is_even(5));
    println!("Result is {:?}", is_even(24));
}

fn is_even(num: i32) -> Option<bool> {
    if num % 2 == 0 {
        Some(true)
    } else {
        None
    }
}
