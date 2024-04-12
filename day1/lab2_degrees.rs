fn main() {
    let temp_c = 32.0;
    let temp_f = convert_celsius_to_fahrenheit(temp_c);
    println!("--Temperatures--");
    println!("Celcius: {}", temp_c);
    println!("Fahrenheit: {}", temp_f);
}

// Function that converts celsius to fahrenheit
fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
