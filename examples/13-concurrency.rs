use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5).map(|i| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
            println!("Thread #{} incremented counter!", i);
            thread::sleep(Duration::from_millis(500));
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
