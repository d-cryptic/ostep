use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // check if we are the child process
    if args.len() > 1 && args[1] == "--child" {
        println!("child (pid:{})", process::id());
        return;
    }

    // parent process
    println!("hello (pid:{})", process::id());

    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            let child_id = child.id();

            // wait for child to complete (equivalent to wait())
            match child.wait() {
                Ok(status) => {
                    println!(
                        "parent of {} (exit_status:{}) (pid:{})",
                        child_id,
                        status.code().unwrap_or(-1),
                        process::id()
                    );
                }
                Err(e) => {
                    eprintln!("wait failed: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("spawn failed: {}", e);
            process::exit(1);
        }
    }
}
