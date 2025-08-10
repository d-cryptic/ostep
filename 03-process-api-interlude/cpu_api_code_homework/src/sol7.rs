use std::ffi::CString;
use std::process;

fn main() {
    println!("Testing stdout closure in child process");

    unsafe {
        let pid = libc::fork();

        if pid == 0 {
            // child process
            println!("Child: Before closing stdout (you should see this)");

            // close standard output
            let result = libc::close(libc::STDOUT_FILENO);
            if result == 0 {
                // this won't be visible since stdout is closed
                println!(
                    "Child: Successfully closed stdout (you should NOT see 
  this)"
                );
            }

            // Try to print after closing stdout
            println!(
                "Child: Attempting to print after closing stdout 
  (invisible)"
            );

            // use printf directly to test C-style output
            let msg =
                CString::new("Chil: Using printf after closing stdout (also invisible)\n").unwrap();
            libc::printf(msg.as_ptr());

            // try writing to stderr instead (should work)
            eprintln!("Child: Writing to stderr (you should see this)");

            // try reopening stdout to /dev/null
            let devnull = CString::new("/dev/null").unwrap();
            let fd = libc::open(devnull.as_ptr(), libc::O_WRONLY);
            if fd == libc::STDOUT_FILENO {
                println!("Child: Reopened stdout to /dev/null (still invisible)");
            }

            process::exit(0);
        } else if pid > 0 {
            // parent process
            println!("Parent: Child created with PID {}", pid);

            // wait for child to complete
            let mut status = 0;
            let waited_pid = libc::wait(&mut status);
            println!("Parent: Child {} finished", waited_pid);
        } else {
            eprintln!("Fork failed");
        }
    }
}
