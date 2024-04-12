// Create a macro to implement a function for generate multipler of a number
macro_rules! multipler {
    ($number:expr) => {
        $number * 5
    };
}

fn main() {
    let num = 7;
    let mul_num = multipler!(num);
    println!(
        "The multipler generated for 5 for the given number {} is {}",
        num, mul_num
    );
}
