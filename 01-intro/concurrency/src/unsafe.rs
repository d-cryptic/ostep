use std::env;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

mod common;

// Using a regular shared variable (not atomic) to show race conditions
use std::cell::UnsafeCell;

struct UnsafeCounter {
    value: UnsafeCell<i32>,
}

unsafe impl Sync for UnsafeCounter {}

impl UnsafeCounter {
    fn new(val: i32) -> Self {
        UnsafeCounter {
            value: UnsafeCell::new(val),
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }

    fn increment(&self) {
        unsafe {
            let ptr = self.value.get();
            *ptr += 1;
        }
    }
}

fn worker(counter: Arc<UnsafeCounter>, loops: usize) {
    for _ in 0..loops {
        counter.increment();
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

    let counter = Arc::new(UnsafeCounter::new(0));

    println!("Initial value : {}", counter.get());

    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    let t1 = thread::spawn(move || worker(counter1, loops));
    let t2 = thread::spawn(move || worker(counter2, loops));

    t1.join().expect("Thread 1 panicked");
    t2.join().expect("Thread 2 panicked");

    println!("Final value   : {}", counter.get());
}
