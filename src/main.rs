use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	// handle errors like this in main
	if let Err(e) = plantasia::run(args) {
		println!("Application Error: {}", e);
		process::exit(1);
	}
}


