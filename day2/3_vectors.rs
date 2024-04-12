fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("Numbers: {:?}", numbers);

    let first_number = numbers[0];
    let second_number = numbers[1];

    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);

    let del_element = numbers.remove(0);
    println!("Numbers: {:?}", numbers);
    println!("Deleted element: {}", del_element);
    println!("Num Elements: {}", numbers.len());
}
