use std::process;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use libc;

fn main() {
    let x = Arc::new(AtomicI32::new(100));
    println!("Before fork: x = {}", x.load(Ordering::SeqCst));

    unsafe {
        let pid = libc::fork();
        if pid == 0 {
            // child process
            println!(
                "Child: x={} (PID:{})",
                x.load(Ordering::SeqCst),
                process::id()
            );
            x.store(200, Ordering::SeqCst);
            println!("Child: changed x to {}", x.load(Ordering::SeqCst));
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("Child: final x = {}", x.load(Ordering::SeqCst));
        } else if pid > 0 {
            // parent process
            println!(
                "Parent: x = {} (PID: {})",
                x.load(Ordering::SeqCst),
                process::id()
            );
            x.store(300, Ordering::SeqCst);
            println!("Parent: changed x to {}", x.load(Ordering::SeqCst));
            std::thread::sleep(std::time::Duration::from_millis(50));
            println!("Parent: final x = {}", x.load(Ordering::SeqCst));

            // wait for child
            let mut status = 0;
            libc::wait(&mut status);
        } else {
            eprintln!("Fork failed");
        }
    }
}
