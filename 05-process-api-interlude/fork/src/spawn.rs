use std::process::{self, Command};

fn main() {
    println!("hello (pid:{})", process::id());

    match Command::new(std::env::current_exe().unwrap())
        .arg("--child")
        .spawn()
    {
        Ok(mut child) => {
            // parent process
            println!("parent of {} (pid:{})", child.id(), process::id());
            let _ = child.wait(); //wait for child to finish
        }

        Err(e) => {
            eprintln!("Spawn failed: {}", e);
            process::exit(1);
        }
    }
}
