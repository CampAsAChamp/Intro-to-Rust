fn main() {
    // Create a HashSet and insert some values
    let mut set = std::collections::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    // Iterate over the values in the set
    for value in &set {
        println!("Value: {}", value);
    }

    println!("");

    println!("{}", set.contains(&4));
    println!("{}", set.contains(&8));
    println!("");

    btreeset()
}

fn btreeset() {
    // Create a BTreeSet and insert some values
    let mut set = std::collections::BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    // Iterate over the values in the set
    for value in &set {
        println!("Value: {}", value);
    }

    println!("");
}
