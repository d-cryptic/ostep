use std::env;
use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <memory_in_mb> [duration_in_seconds]", args[0]);
        std::process::exit(1);
    }

    // parse memory size in MBs
    let memory_mb: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid memory size");
        std::process::exit(1);
    });

    // parse original duration in seconds
    let duration_seconds: Option<u64> = if args.len() >= 3 {
        Some(args[2].parse().unwrap_or_else(|_| {
            eprintln!("Error: Invalid duration");
            std::process::exit(1);
        }))
    } else {
        None
    };

    // calculate array size (1 MB = 1024*1024 bytes)
    // using u8 array, so each element is 1 byte
    let array_size = memory_mb * 1024 * 1024;
    println!("Allocating {} MB of memory...", memory_mb);

    // Allocate memory
    let mut memory: Vec<u8> = vec![0u8; array_size];
    println!("Memory allocated. Starting to access memory continuously...");

    let start_time = Instant::now();
    let mut iteration = 0u64;

    loop {
        // stream through the array, touching each entry
        for i in 0..array_size {
            memory[i] = (memory[i].wrapping_add(1)) % 255;
        }

        iteration += 1;

        // check if we should stop (if duration was specified)
        if let Some(duration) = duration_seconds {
            if start_time.elapsed().as_secs() >= duration {
                println!("time limit reached. Completed {} iterations.", iteration);
                break;
            }
        }

        // print status every 10 iterations
        if iteration % 10 == 0 {
            println!("Iteration {}: Still accessing memory...", iteration);
        }

        // small sleep to prevent cpu from being completely consumed
        sleep(Duration::from_millis(10));
    }

    println!("Program finished.");
}
