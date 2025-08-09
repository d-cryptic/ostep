use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::process;

fn get_time() -> f64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

    now.as_secs() as f64 + now.subsec_nanos() as f64/1e9
}

fn spin(seconds: u64) {
    let start = get_time();
    while (get_time() - start) < seconds as f64 {
        // Busy wait
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: mem <value>");
        process::exit(1);
    }

    let initial_value: i32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: '{}' is not a valid integer", args[1]);
        process::exit(1);
    });

    // allocate memory on heap (Box is Rust's equivalent to malloc)
    let mut p = Box::new(initial_value);

    println!("({}) addr pointed to by p: {:p}", process::id(), p.as_ref());

    loop {
        spin(1);
        *p += 1;
        println!("({}) value of p: {}", process::id(), *p);
    }
}