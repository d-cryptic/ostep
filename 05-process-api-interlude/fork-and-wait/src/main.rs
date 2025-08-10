use std::process;

fn main() {
    println!("hello pid:{}", process::id());

    let rc = unsafe { libc::fork() };

    match rc {
        -1 => {
            // fork failed; exit
            eprintln!("fork failed");
            process::exit(1);
        }
        0 => {
            // child (new process)
            println!("child (pid:{})", process::id());
        }
        child_pid => {
            let mut status = 0;
            let rc_wait = unsafe { libc::wait(&mut status) };
            println!(
                "parent of {} (rc_wait:{}) (pid:{})",
                child_pid,
                rc_wait,
                process::id()
            )
        }
    }
}
