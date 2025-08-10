use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    // open file before fork
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("shared_file.txt")
        .expect("Failed to open file");

    let fd = file.as_raw_fd();
    println!("Opened file with fd: {}", fd);

    unsafe {
        let pid = libc::fork();

        if pid == 0 {
            // child process
            println!("Child (PID: {}): Can access fd {}", process::id(), fd);

            for i in 0..5 {
                writeln!(file, "Child line {}", i).expect("Child write failed");
                file.flush().expect("Child flush failed");
                thread::sleep(Duration::from_millis(100));
            }
        } else if pid > 0 {
            // parent process
            println!("Parent (PID: {}): Can access fd {}", process::id(), fd);

            for i in 0..5 {
                writeln!(file, "Parent line {}", i).expect("Parent write failed");
                file.flush().expect("Parent flush failed");
                thread::sleep(Duration::from_millis(100));
            }

            // wait for child
            let mut status = 0;
            libc::wait(&mut status);
        } else {
            eprintln!("Fork failed");
        }
    }
}
