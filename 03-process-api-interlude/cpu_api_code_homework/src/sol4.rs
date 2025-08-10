use std::ffi::CString;
use std::process;
use std::ptr;

fn main() {
    println!("Testing different exec() variants:");

    // execl() -arguments as list
    test_exec("execl", || unsafe {
        let program = CString::new("/bin/ls").unwrap();
        let arg0 = CString::new("ls").unwrap();
        let arg1 = CString::new("-l").unwrap();
        libc::execl(
            program.as_ptr(),
            arg0.as_ptr(),
            arg1.as_ptr(),
            ptr::null::<i8>(),
        )
    });

    // execv() - arguments as vector
    test_exec("execv", || unsafe {
        let program = CString::new("/bin/ls").unwrap();
        let arg0 = CString::new("ls").unwrap();
        let arg1 = CString::new("-l").unwrap();
        let args = [arg0.as_ptr(), arg1.as_ptr(), ptr::null()];
        libc::execv(program.as_ptr(), args.as_ptr())
    });

    // execlp() - arguments as list, PATH search
    test_exec("execlp", || unsafe {
        let program = CString::new("ls").unwrap();
        let arg0 = CString::new("ls").unwrap();
        let arg1 = CString::new("-l").unwrap();
        libc::execlp(
            program.as_ptr(),
            arg0.as_ptr(),
            arg1.as_ptr(),
            ptr::null::<i8>(),
        )
    });

    // execvp() - arguments as vector, PATH search
    test_exec("execvp", || unsafe {
        let program = CString::new("ls").unwrap();
        let arg0 = CString::new("ls").unwrap();
        let arg1 = CString::new("-l").unwrap();
        let args = [arg0.as_ptr(), arg1.as_ptr(), ptr::null()];
        libc::execvp(program.as_ptr(), args.as_ptr())
    });
}

fn test_exec<F>(name: &str, exec_fn: F)
where
    F: FnOnce() -> i32,
{
    unsafe {
        let pid = libc::fork();

        if pid == 0 {
            // Child process
            println!("\n--- Testing {} ---", name);
            exec_fn();
            // If we reach here, exec failed
            eprintln!("{} failed", name);
            process::exit(1);
        } else if pid > 0 {
            // Parent process - wait for child
            let mut status = 0;
            libc::wait(&mut status);
        } else {
            eprintln!("Fork failed for {}", name);
        }
    }
}
