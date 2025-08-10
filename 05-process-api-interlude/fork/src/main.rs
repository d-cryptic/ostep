use std::process;

fn main() {
    println!("hello (pid: {})", process::id());

    match unsafe { libc::fork() } {
        -1 => {
            // fork failed
            eprintln!("fork failed");
            process::exit(1);
        }

        0 => {
            //child (new process)
            println!("child (pid:{})", process::id());
        }

        child_pid => {
            // parent goes down this path (main)
            println!("parent of {} (pid:{})", child_pid, process::id());
        }
    }
}
