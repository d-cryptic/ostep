use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing wait() behavior");

    unsafe {
        let pid = libc::fork();

        if pid == 0 {
            // Child process
            println!("Child: PID = {}", process::id());
            println!("Child: Sleeping for 2 seconds...");
            thread::sleep(Duration::from_secs(2));
            println!("Child: About to exit with status 42");

            // Test wait() in child - this should fail
            println!("Child: Calling wait()...");
            let mut status = 0;
            let result = libc::wait(&mut status);
            if result == -1 {
                println!("Child: wait() failed (expected - no children to wait for)");
            } else {
                println!("Child: wait() returned: {}", result);
            }

            process::exit(42); //exit with custom status
        } else if pid > 0 {
            // Parent process
            println!("Parent: PID = {}, Child PID = {}", process::id(), pid);
            println!("Parent: Calling wait()...");

            let mut status = 0;
            let waited_pid = libc::wait(&mut status);

            println!("Parent: wait() returned PID: {}", waited_pid);
            println!("Parent: Raw status: {}", status);

            // extract exit status (assuming normal termination)
            if libc::WIFEXITED(status) {
                let exit_status = libc::WEXITSTATUS(status);
                println!("Parent: Child exited normally with status: {}", exit_status);
            } else if libc::WIFSIGNALED(status) {
                let signal = libc::WTERMSIG(status);
                println!("Parent: Child terminated by signal: {}", signal);
            }
        } else {
            eprintln!("Fork failed");
        }
    }
}
