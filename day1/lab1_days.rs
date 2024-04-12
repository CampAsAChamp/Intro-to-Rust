use std::io;

fn main() {
    println!("Please enter a number for the day [1-7]: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let day_number: i32 = input.trim().parse().expect("Invalid input");

    let day = match day_number {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid input: Number must be between 1 and 7.",
    };

    println!("Today is {}, input = {}", day, day_number);
}
