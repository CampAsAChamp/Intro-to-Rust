use wasm_bindgen::prelude::*;

// Convert packages into web packages
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
