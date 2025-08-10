use process::Command;
use std::process;

// handle child process execution
fn child_main() {
    println!("Child (pid:{})", process::id());
}

// check if we are the child process
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--child" {
        child_main();
        return;
    }

    println!("hello (pid:{})", process::id());

    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            println!("parent of {} (pid:{})", child.id(), process::id());
            let _ = child.wait();
        }

        Err(e) => {
            eprintln!("spawn failed: {}", e);
            process::exit(1);
        }
    }
}
