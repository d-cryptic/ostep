use std::ffi::c_uint;
use std::ffi::CString;
use std::process;
use std::ptr;

fn main() {
    let rc = unsafe { libc::fork() };

    match rc {
        -1 => {
            // fork failed; exit
            eprintln!("fork failed");
            process::exit(1);
        }

        0 => {
            // child: redirect standard output to a file
            unsafe {
                libc::close(libc::STDOUT_FILENO);
                let fd = libc::open(
                    CString::new("src/tmp.txt").unwrap().as_ptr(),
                    libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC,
                    libc::S_IRWXU as c_uint,
                );
                assert!(fd >= 0);
            }

            // now exec "wc"
            let program = CString::new("wc").unwrap();
            let arg1 = CString::new("src/main.rs").unwrap();

            let args = [program.as_ptr(), arg1.as_ptr(), ptr::null()];

            unsafe {
                libc::execvp(program.as_ptr(), args.as_ptr());
            }
        }

        _child_pid => {
            // parent goes down this path (original process)
            let wc = unsafe { libc::wait(ptr::null_mut()) };
            assert!(wc >= 0);
        }
    }
}
