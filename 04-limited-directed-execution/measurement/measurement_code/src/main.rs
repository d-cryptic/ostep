use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::process;
use std::time::{Duration, Instant};

fn main() {
    println!("System Call and Context Switch Cost Measurement\n");

    // test timer precision
    let timer_precision = measure_timer_precision();
    println!("Timer precision: {} nanoseconds", timer_precision);

    // Measure system call cost
    let syscall_cost = measure_syscall_cost(timer_precision);
    println!("System call cost: {} nanoseconds", syscall_cost);

    // Measure context switch cost
    let context_switch_cost = measure_context_switch_cost();
    println!("Context switch cost: {} nanoseconds", context_switch_cost);
}

fn measure_timer_precision() -> u64 {
    const ITERATIONS: usize = 1000;
    let mut min_diff = u64::MAX;
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        let end = Instant::now();
        let diff = end.duration_since(start).as_nanos() as u64;
        if diff > 0 && diff < min_diff {
            min_diff = diff;
        }
    }

    min_diff
}

fn measure_syscall_cost(timer_precision: u64) -> u64 {
    // calculate iterations needed for good measurement
    let iterations = std::cmp::max(100_000, timer_precision * 1000);

    // open /dev/zero for 0 byte reads
    let mut file = File::open("/dev/zero").expect("Failed to open /dev/zero");
    let mut buffer = [0u8; 0];

    for _ in 0..1000 {
        let _ = file.read(&mut buffer);
    }

    // measure
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = file.read(&mut buffer);
    }
    let end = Instant::now();

    let total_time = end.duration_since(start).as_nanos() as u64;
    total_time / iterations
}

fn measure_context_switch_cost() -> u64 {
    const ITERATIONS: usize = 100_000;

    // create pipes for communiation
    let mut pipe1 = [0; 2];
    let mut pipe2 = [0; 2];

    unsafe {
        if libc::pipe(pipe1.as_mut_ptr()) == -1 || libc::pipe(pipe2.as_mut_ptr()) == -1 {
            panic!("Failed to create pipes");
        }
    }

    let (read1, write1) = (pipe1[0], pipe1[1]);
    let (read2, write2) = (pipe2[0], pipe2[1]);

    // set cpu affinity to cpu 0
    // set_cpu_affinity(0);

    let pid = unsafe { libc::fork() };

    if pid == 0 {
        // child process
        unsafe {
            libc::close(write1);
            libc::close(read2);
        }

        let mut buffer = [1u8; 1];

        for _ in 0..ITERATIONS {
            // Read from pipe1
            unsafe { libc::read(read1, buffer.as_mut_ptr() as *mut libc::c_void, 1) };

            // Write to pipe2
            unsafe { libc::write(write2, buffer.as_ptr() as *const libc::c_void, 1) };
        }

        process::exit(0);
    } else {
        // parent process
        unsafe {
            libc::close(read1);
            libc::close(write2);
        }

        let mut buffer = [1u8; 1];
        let start = Instant::now();

        for _ in 0..ITERATIONS {
            // write to pipe1
            unsafe { libc::write(write1, buffer.as_ptr() as *const libc::c_void, 1) };
            // Read from pipe2
            unsafe { libc::read(read2, buffer.as_mut_ptr() as *mut libc::c_void, 1) };
        }

        let end = Instant::now();

        // wait for child
        let mut status = 0;
        unsafe { libc::wait(&mut status) };

        // each iteration involves 2 context switches (parent->child, child->parent)
        let total_time = end.duration_since(start).as_nanos() as u64;
        total_time / (ITERATIONS as u64 * 2)
    }
}

// fn set_cpu_affinity(cpu: usize) {
//     unsafe {
//         let mut cpu_set: libc::cpu_set_t = std::mem::zeroed();
//         libc::CPU_SET(cpu, &mut cpu_set);

//         let result = libc::sched_setaffinity(
//             0, //current process
//             std::mem::size_of::<libc::cpu_set_t>(),
//             &cpu_set,
//         );

//         if result != 0 {
//             eprintln!("Warning: Failed to set CPU affinity");
//         }
//     }
// }
