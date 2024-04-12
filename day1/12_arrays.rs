fn main() {
    // let arr: [i32; 5] = [5, 7, 9, 11, 13];
    // let arr = [5, 7, 9, 11, 13];
    let arr = [-1; 5];
    println!("Array elements are {:?}", arr);
    println!("Array size is {}", arr.len());

    for i in 0..arr.len() {
        println!("Element at index {} is {}", i, arr[i]);
    }

    for val in arr.iter() {
        println!("Element is {}", val);
    }
}
