use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    unsafe {
        let pid = libc::fork();

        if pid == 0 {
            // child process prints first
            println!("hello");
        } else if pid > 0 {
            // parent process - add delay to ensure child prints first
            thread::sleep(Duration::from_millis(10));
            println!("goodbye");
        } else {
            eprintln!("Fork failed");
        }
    }
}
