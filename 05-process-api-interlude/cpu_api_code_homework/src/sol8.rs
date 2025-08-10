use std::ffi::CString;
use std::process;

fn main() {
    println!("Creating pipe between two children");

    unsafe {
        // create pipe
        let mut pipe_fds = [0; 2];
        if libc::pipe(pipe_fds.as_mut_ptr()) == -1 {
            eprintln!("Failed to create pipe");
            return;
        }

        let read_fd = pipe_fds[0];
        let write_fd = pipe_fds[1];
        println!(
            "Parent: Created pipe - read_fd: {}, write_fd: {}",
            read_fd, write_fd
        );

        // create first child (writer)
        let child1 = libc::fork();
        if child1 == 0 {
            // Child 1: Writer process
            println!("Child 1 (writer): PID = {}", process::id());

            // close read end on pipe
            libc::close(read_fd);

            // redirect stdout to write end of pipe
            libc::dup2(write_fd, libc::STDOUT_FILENO);
            libc::close(write_fd); //close original write_fd

            // write some data
            println!("Hello from child 1!");
            println!("This is line 2 from writer");
            println!("Final line from writer process");

            process::exit(0);
        }

        // create second child (reader)
        let child2 = libc::fork();
        if child2 == 0 {
            // child 2 : Reader process
            println!("Child 2 (reader): PID = {}", process::id());

            // close write end of pipe
            libc::close(write_fd);

            // redirect stdin to read end of pipe
            libc::dup2(read_fd, libc::STDIN_FILENO);
            libc::close(read_fd); //close original read_fd

            //Execute a program that reads from stdin (like 'cat')
            let program = CString::new("/bin/cat").unwrap();
            let arg0 = CString::new("cat").unwrap();
            libc::execl(program.as_ptr(), arg0.as_ptr(), std::ptr::null::<i8>());

            // if exec fails
            eprintln!("Child 2: exec failed");
            process::exit(1);
        }

        // parent process
        println!(
            "Parent: Created writer child {} and reader child {}",
            child1, child2
        );

        // close both ends of pipe in parent
        libc::close(read_fd);
        libc::close(write_fd);

        // wait for both children
        let mut status = 0;
        let pid1 = libc::wait(&mut status);
        println!("Parent: Child {} finished", pid1);

        let pid2 = libc::wait(&mut status);
        println!("Parent: Child {} finished", pid2);
    }
}
