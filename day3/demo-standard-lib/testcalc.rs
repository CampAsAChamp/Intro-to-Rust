fn main() {
    let sum = calc::sum(10, 2);
    let diff = calc::diff(10, 2);
    let mul = calc::multiply(10, 2);
    let div = calc::divide(10, 2);

    println!("The sum is {}", sum);
    println!("The diff is {}", diff);
    println!("The mul is {}", mul);
    println!("The div is {}", div);
}
