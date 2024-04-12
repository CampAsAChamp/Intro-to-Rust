fn main() {
    // hello();
    // println!("{}", sum(1, 2));
    let mut n = 10;

    convert_to_zero_by_value(n);
    println!("{}", n);

    convert_to_zero_by_ref(&mut n);
    println!("{}", n);
}

fn hello() {
    println!("Hello Rust");
}

fn sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

// Call by value
fn convert_to_zero_by_value(mut num: i32) {
    num = 0;
    println!("Value of num is {}", num);
}

// Call by reference
fn convert_to_zero_by_ref(num: &mut i32) {
    *num = 0;
    println!("Value of num is {}", *num);
}
