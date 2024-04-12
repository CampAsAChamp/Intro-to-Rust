fn main() {
    for i in 1..10 {
        println!("{}", i);
    }

    println!("");

    let mut i = 1;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    println!("");

    let mut x = 1;
    loop {
        println!("{}", x);
        x += 1;
        if x > 10 {
            break;
        }
    }
}
