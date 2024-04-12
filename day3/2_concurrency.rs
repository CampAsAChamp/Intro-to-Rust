use std::thread;
use std::time::Duration;

fn sum_range(start: usize, end: usize) -> usize {
    let mut sum = 0;
    for i in start..=end {
        sum += i;
    }
    return sum;
}

fn basic_thread_example() {
    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawn thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=5 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(100));
    }

    t1.join().unwrap();
}

fn main() {
    const N: usize = 100_000;
    const NUM_THREADS: usize = 2;

    let mut handles = vec![];

    for i in 0..NUM_THREADS {
        let start = i * (N / NUM_THREADS as usize) + 1;
        let end = (i + 1) * (N / NUM_THREADS);
        let handle = thread::spawn(move || sum_range(start, end));
        handles.push(handle);
    }

    let mut final_sum = 0;
    for handle in handles {
        final_sum += handle.join().unwrap();
    }

    println!("Final sum: {}", final_sum);
}
