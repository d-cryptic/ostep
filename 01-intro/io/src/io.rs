use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./io.txt")
        .expect("Failed to open ./io.txt");

    let buffer = "hello world\n";

    file.write_all(buffer.as_bytes())
        .expect("Failed to write to file");

    file.sync_all().expect("Failed to sync file");

    // File automatically closed when dropped
}
