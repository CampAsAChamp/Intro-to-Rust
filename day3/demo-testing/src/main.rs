// Make a unique module name
mod tests;

fn factorial(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..n + 1 {
        result = result * i;
    }
    return result;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let n = 5;
    println!("factorial({}) = {}", n, factorial(n));

    let n = 6;
    println!("factorial({}) = {}", n, factorial(n));

    let n = 7;
    println!("factorial({}) = {}", n, factorial(n));
}
