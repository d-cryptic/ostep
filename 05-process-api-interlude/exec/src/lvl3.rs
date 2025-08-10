use std::process::{self, Command};

fn main() {
    println!("hello (pid:{})", process::id());

    // fork equivalent: spawn a new process that runs wc
    match Command::new("wc").arg("src/main.rs").spawn() {
        Ok(mut child) => {
            let child_id = child.id();
            println!("child process started (pid:{})", child_id);

            // Parent waits for child (equivalent to wait())
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
