fn main() {
    let mut names: Vec<String> = Vec::new();

    let mut line = String::new();

    while line.trim().to_string() != "q" {
        println!("Enter a name (or press 'q' to stop): ");
        line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim().to_string() != "q" {
            names.push(line.trim().to_string());
        }
    }

    println!("Names: {:?}", names);
}
