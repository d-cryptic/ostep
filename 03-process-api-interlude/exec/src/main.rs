use std::ffi::CString;
use std::process;
use std::ptr;

fn main() {
    println!("hello (pid:{})", process::id());

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

            // prepare arguments for execvp
            let program = CString::new("wc").unwrap();
            let arg1 = CString::new("src/main.rs").unwrap();

            let args = [program.as_ptr(), arg1.as_ptr(), ptr::null()];

            // execute word count program
            unsafe {
                libc::execvp(program.as_ptr(), args.as_ptr());
            }

            // This shouldn't print out (only if exec fails)
            println!("this shouldn't print out");
            process::exit(1); // Exit if exec fails
        }

        child_pid => {
            // parent goes down this path
            let mut status = 0;
            let rc_wait = unsafe { libc::wait(&mut status) };
            println!(
                "parent of {} (rc_wait:{}) (pid:{})",
                child_pid,
                rc_wait,
                process::id()
            );
        }
    }
}
