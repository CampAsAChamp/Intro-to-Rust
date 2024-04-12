fn main() {
    let mut marks = std::collections::HashMap::new();
    marks.insert("Rust", 100);
    marks.insert("Python", 90);
    marks.insert("Java", 80);

    let java_marks = marks.get("Java");
    println!("Java marks: {:?}", java_marks);

    let has_java = marks.contains_key("Java");
    println!("Has Java: {:?}", has_java);

    for (key, value) in &marks {
        println!("Key: {}, Value: {}", key, value);
    }

    println!("");
    btree();
}

fn btree() {
    let mut marks = std::collections::BTreeMap::new();
    marks.insert("Rust", 100);
    marks.insert("Python", 90);
    marks.insert("Java", 80);
    marks.insert("C++", 50);
    marks.insert("JavaScript", 60);
    marks.insert("Ruby", 55);

    let java_marks = marks.get("Java");
    println!("Java marks: {:?}", java_marks);

    let has_java = marks.contains_key("Java");
    println!("Has Java: {:?}", has_java);

    for (key, value) in &marks {
        println!("Key: {}, Value: {}", key, value);
    }
}
