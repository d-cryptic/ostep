mod lib;

use lib::spin;
use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("usage: {} <string>", args[0]);
		process::exit(1);
	}

	let message = &args[1];

	loop {
		spin(1);
		println!("{}", message);
	}
}