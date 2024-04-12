use std::io;
fn main() {
    // User enters name and phone number
    let mut name = String::new();
    let mut phone = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter your phone number: ");
    io::stdin()
        .read_line(&mut phone)
        .expect("Failed to read line");

    // Add name and phone number to a hashmap
    let mut phonebook = std::collections::HashMap::new();
    // Trim name and phone number before inserting into hashmap

    phonebook.insert(name.trim().to_string(), phone.trim().to_string());

    // Display the hashmap
    println!("Phonebook: {:?}", phonebook);
}
