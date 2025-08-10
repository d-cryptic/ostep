use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we're the child process
    if args.len() > 1 && args[1] == "--child" {
        println!("child (pid:{})", process::id());

        // Execute word count program - this replaces the current process
        let status = Command::new("wc")
            .arg("src/main.rs")
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to execute wc: {}", e);
                process::exit(1);
            });

        // Exit with the same code as the executed program
        process::exit(status.code().unwrap_or(1));
    }

    // parent process
    println!("hello (pid:{})", process::id());

    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            let child_id = child.id();
            // wait for child to complete
            match child.wait() {
                Ok(_status) => {
                    println!(
                        "parent of {} (rc_wait:{}) (pid:{})",
                        child_id,
                        child_id,
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
