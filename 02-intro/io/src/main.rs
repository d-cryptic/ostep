use std::fs::OpenOptions;
use std::io::Write;
use std::process;

fn main() {
    // Open file with create, write-only, truncate flags
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./tmp.txt")
        .unwrap_or_else(|err| {
            eprintln!("Failed to open ./tmp.txt: {}", err);
            process::exit(1);
        });

    let buffer = "hello world\n";

    // write to file
    match file.write_all(buffer.as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to write to file: {}", err);
            process::exit(1);
        }
    }

    // sync file to disk (equivalent to fsync)
    match file.sync_all() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to sync file: {}", err);
            process::exit(1);
        }
    }

    // File is automatically closed when it goes out of scope
    println!("Successfully wrote to ./tmp.txt");
}
