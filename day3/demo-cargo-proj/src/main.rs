use rayon::prelude::*;

fn sum_of_square_seq(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    for num in start..=end {
        sum = sum + num * num as u64;
        // sum += i * i; as u64;
    }
    return sum;
}

fn sum_of_square_parallel(start: u64, end: u64) -> u64 {
    // Into parrallel iterator, map
    return (start..=end)
        .into_par_iter()
        .map(|num| num * num as u64)
        .sum();
}

fn main() {
    let start = 1;
    let end = 1_000_000;

    let start_time = std::time::Instant::now();
    let sum_seq = sum_of_square_seq(start, end);
    let end_time_seq = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let sum_par = sum_of_square_parallel(start, end);
    let end_time_par = start_time.elapsed();

    println!("Result of sum_of_square_seq: {}", sum_seq);
    println!("Result of sum_of_square_parallel: {}", sum_par);
    println!("Elapsed time of sum_of_square_seq: {:?}", end_time_seq);
    println!("Elapsed time of sum_of_square_parallel: {:?}", end_time_par);
}
