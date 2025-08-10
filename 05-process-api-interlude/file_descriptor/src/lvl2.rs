use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    // create output file and redirect stdout to it
    let output_file = File::create("src/tmp.txt").unwrap_or_else(|e| {
        eprintln!("failed to create output file: {}", e);
        std::process::exit(1);
    });

    // execute wc with stdout redirected to file
    let status = Command::new("wc")
        .arg("src/main.rs")
        .stdout(Stdio::from(output_file))
        .status()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute wc: {}", e);
            std::process::exit(1);
        });

    assert!(status.process());

    println!("Word count output written to src/tmp.txt");
}
