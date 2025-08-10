use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing waitpid() behavior");

    unsafe {
        // Create multiple children to demonstrate waitpid's selectivity
        let child1 = libc::fork();
        if child1 == 0 {
            println!("Child 1: PID = {}, sleeping 1 second", process::id());
            thread::sleep(Duration::from_secs(1));
            println!("Child 1: Exiting with status 10");
            process::exit(10);
        }

        let child2 = libc::fork();
        if child2 == 0 {
            println!("Child 2: PID = {}, sleeping 3 seconds", process::id());
            thread::sleep(Duration::from_secs(3));
            println!("Child 2: Exiting with status 20");
            process::exit(20);
        }

        // Parent process
        println!("Parent: Created children {} and {}", child1, child2);

        // wait for specific child (child2) using waitpid
        println!(
            "Parent: Waiting specifically for child2 (PID: {}) using waitpid",
            child2
        );
        let mut status = 0;
        let waited_pid = libc::waitpid(child2, &mut status, 0);
        println!("Parent: waitpid() returned PID: {}", waited_pid);
        if libc::WIFEXITED(status) {
            let exit_status = libc::WEXITSTATUS(status);
            println!(
                "Parent: Child {} exited with status: {}",
                waited_pid, exit_status
            );
        }

        // Now wait for any remaining child
        println!("Parent: Waiting for remaining child with waitpid(-1, ...)");

        let waited_pid2 = libc::waitpid(-1, &mut status, 0);
        println!("Parent: Second waitpid() returned PID: {}", waited_pid2);
        if libc::WIFEXITED(status) {
            let exit_status = libc::WEXITSTATUS(status);
            println!(
                "Parent: Child {} exited with status: {}",
                waited_pid2, exit_status
            );
        }

        // demonstrate non blocking wait
        println!("Parent: Testing non-blocking waitpid with WNOHANG");
        let result = libc::waitpid(-1, &mut status, libc::WNOHANG);
        if result == 0 {
            println!("Parent: no children available (WNOHANG returned 0)");
        } else if result == -1 {
            println!("Parent: no more children to wait for");
        }
    }
}
