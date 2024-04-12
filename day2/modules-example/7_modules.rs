mod my_module;

pub mod shape {
    pub fn draw(name: String) {
        println!("Drawing a {}!", name);
    }
}

pub mod math {
    pub mod geo {
        pub mod shape {
            pub fn draw_sh(name: String) {
                println!("Drawing Shape using Nested Modules {}", name);
            }
        }
    }
}

use shape::draw;

use my_module::another_module::my_another_function;
use my_module::my_function;
use my_module::nested_module::my_nested_function;

fn main() {
    shape::draw("Square".to_string());
    shape::draw("Circle".to_string());
    draw("Triangle".to_string());

    math::geo::shape::draw_sh("Square".to_string());

    my_function();
    my_nested_function();
    my_another_function();
}
