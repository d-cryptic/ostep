use std::env;
use std::fs::File;
use std::process::{self, Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we're the child process
    if args.len() > 1 && args[1] == "--child" {
        // Child process: redirect output and exec wc
        let output_file = File::create("src/tmp.txt").unwrap_or_else(|e| {
            eprintln!("failed to create output file: {}", e);
            process::exit(1);
        });

        let status = Command::new("wc")
            .arg("p4.c")
            .stdout(Stdio::from(output_file))
            .status()
            .unwrap_or_else(|e| {
                eprintln!("failed to execute wc: {}", e);
                process::exit(1);
            });

        process::exit(status.code().unwrap_or(1));
    }

    // parent process: spawn child
    match Command::new(&args[0]).arg("--child").spawn() {
        Ok(mut child) => {
            // wait for child to complete
            match child.wait() {
                Ok(status) => {
                    assert!(status.success());
                    println!("Child process completed successfully");
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
