use std::env;
use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    // Print PID for convenience
    let pid = process::id();
    println!("Process ID (PID): {}", pid);
    println!("You can examine this process with: pmap -x {}", pid);
    println!("----------------------------------------");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <memory_in_mb> [duration_in_seconds]", args[0]);
        process::exit(1);
    }

    let memory_mb: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid memory size");
        process::exit(1);
    });

    let duration_seconds: Option<u64> = if args.len() >= 3 {
        Some(args[2].parse().unwrap_or_else(|_| {
            eprintln!("Error: Invalid duration");
            process::exit(1);
        }))
    } else {
        None
    };

    let array_size = memory_mb * 1024 * 1024;

    println!("Allocating {} MB of memory...", memory_mb);
    let mut memory: Vec<u8> = vec![0u8; array_size];
    println!("Memory allocated. Starting to access memory continuously...");

    // Initial touch of all memory
    println!("Initial memory touch...");
    for i in 0..array_size {
        memory[i] = 1;
    }
    println!("Initial touch complete. Check memory map now!");

    let start_time = Instant::now();
    let mut iteration = 0u64;

    loop {
        for i in 0..array_size {
            memory[i] = (memory[i].wrapping_add(1)) % 255;
        }

        iteration += 1;

        if let Some(duration) = duration_seconds {
            if start_time.elapsed().as_secs() >= duration {
                println!("Time limit reached. Completed {} iterations.", iteration);
                break;
            }
        }

        if iteration % 10 == 0 {
            println!(
                "Iteration {}: Still accessing memory (PID: {})",
                iteration, pid
            );
        }

        sleep(Duration::from_millis(100));
    }

    println!("Program finished.");
}
