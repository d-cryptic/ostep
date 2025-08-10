use std::process;

fn main() {
    println!("hello (pid:{})", process::id());

    let rc = unsafe { libc::fork() };

    match rc {
        -1 => {
            eprintln!("fork failed");
            process::exit(1);
        }
        0 => {
            // child process
            println!("child (pid:{})", process::id());
            // Child exits normally
            process::exit(0);
        }
        child_pid => {
            // parent process
            let mut status: libc::c_int = 0;
            let rc_wait = unsafe { libc::wait(&mut status) };

            if rc_wait == -1 {
                eprintln!("wait failed");
                process::exit(1);
            }

            println!(
                "parent of {} (rc_wait:{}) (pid:{})",
                child_pid,
                rc_wait,
                process::id()
            )
        }
    }
}
