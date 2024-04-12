use std::io;

fn main() {
    println!("Please enter a number to calculate the factorial for: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Invalid input");

    let mut factorial = 1;
    for i in 1..(number + 1) {
        factorial *= i;
    }

    println!("The factorial of {}! is {}", number, factorial);
}
