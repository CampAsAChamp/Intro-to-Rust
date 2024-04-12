fn main() {
    let x1 = vec![1, 2, 3];

    println!("{:?}", x1);

    // Ownership has been completely transfered
    let x2 = x1;
    println!("{:?}", x2);

    let x2_return = display_return(x2);
    println!("{:?}", x2_return);
}

fn display(v: Vec<i32>) {
    println!("Inside display{:?}", v);
}

fn display_return(v: Vec<i32>) -> Vec<i32> {
    println!("Inside display{:?}", v);
    return v;
}
