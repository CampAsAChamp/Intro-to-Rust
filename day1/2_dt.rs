fn main() {
    let res = 90;
    let age: u32 = 18;

    let sum: i32 = 10 - 15;

    let m: isize = 10;
    let n: isize = 20;

    println!("value is {}", res);
    println!("value is {}", age);
    println!("value is {}", sum);
    println!("value is {}", m);
    println!("value is {}", n);

    let float_with_separator = 11_000.555_001;
    println!("float value {}", float_with_separator);
    let int_with_separator = 50_000;
    println!("int value {}", int_with_separator);

    let isfun: bool = true;
    println!("Is Rust Programming Fun ? {}", isfun);
    // Character DT
    let special_character = '@'; //default
    let alphabet: char = 'A';
    let emoji: char = '🍪';
    println!("special character is {}", special_character);
    println!("alphabet is {}", alphabet);
    println!("emoji is {}", emoji);
}
