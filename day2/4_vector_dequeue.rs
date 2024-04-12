use std::collections::VecDeque;

fn main() {
    let mut v_deq: VecDeque<i32> = VecDeque::new();

    v_deq.push_back(10);
    v_deq.push_front(20);

    println!("VecDeque: {:?}", v_deq);

    // Will print Some or None, unless you put `unwrap()` on it
    println!("Front: {:?}", v_deq.front().unwrap());
    println!("Back: {:?}", v_deq.back().unwrap());
}
