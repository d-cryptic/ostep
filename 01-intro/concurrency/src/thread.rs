use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

mod common;
use common::*;

fn worker(counter: Arc<Mutex<i32>>, loops: usize) {
    for _ in 0..loops {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: threads <loops>");
        process::exit(1);
    }

    let loops: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid number", args[1]);
        process::exit(1);
    });

    let counter = Arc::new(Mutex::new(0i32));

    println!("Initial value: {}", *counter.lock().unwrap());

    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    let t1 = thread::spawn(move || worker(counter1, loops));
    let t2 = thread::spawn(move || worker(counter2, loops));

    t1.join().expect("Thread 1 panicked");
    t2.join().expect("Thread 2 panicked");

    println!("Final value: {}", *counter.lock().unwrap());
}
