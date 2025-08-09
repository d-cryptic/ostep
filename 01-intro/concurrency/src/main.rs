use std::env;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

mod common;

// using atomic for thread safe counter (equivalent to volatile)
static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn worker(loops: usize) {
    for _ in 0..loops {
        COUNTER.fetch_add(1, Ordering::SeqCst);
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

    println!("Initial value: {}", COUNTER.load(Ordering::SeqCst));

    // Create two threads (equivalent to pthread_create)
    let t1 = thread::spawn(move || worker(loops));
    let t2 = thread::spawn(move || worker(loops));

    // wait for threads to complete (equivalent to pthread_join)
    t1.join().expect("Thread 1 panicked");
    t2.join().expect("Thread 2 panicked");

    println!("Final Value: {}", COUNTER.load(Ordering::SeqCst));
}
