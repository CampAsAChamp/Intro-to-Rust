macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr) => {
        $a
    };
}

macro_rules! unit_converter {
    ($fn_name:ident, $conversion:expr) => {
        fn $fn_name(value: f64) -> f64 {
            return $conversion * value;
        }
    };
}

fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

unit_converter!(inches_to_cm, 2.54);

macro_rules! repeat {
    ($a:expr, $times:expr) => {
        for _ in 0..$times {
            $a
        }
    };
}

fn main() {
    let sum1 = add!(10, 20);
    let sum2 = add_fn(1, 2);
    let sum3 = add!(10);
    println!("The sum is {}", sum1);
    println!("The sum is {}", sum2);
    println!("The sum is {}", sum3);

    let inches = 10.0;
    let cm = inches_to_cm(inches);
    println!("{} inches is {} cm", inches, cm);

    repeat!(println!("Hello World"), 5);
}
