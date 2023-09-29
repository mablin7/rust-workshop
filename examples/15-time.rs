use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    // Simulating some work
    let _ = (0..1_000_000).map(|x| x * 2).collect::<Vec<_>>();

    let duration = start.elapsed();

    println!("Elapsed time is {:?}", duration);
}
