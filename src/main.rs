use std::{env, io};

use chrono::Local;
fn main() {
	let mut args = env::args();
	let _ = args.next(); // program name
	let format = args.next().unwrap_or("[%H:%M:%S]".to_string());

	let mut buf = String::new();
	let stdin = io::stdin();

	loop {
		match stdin.read_line(&mut buf) {
			Ok(0) => break,                                                              // EOF
			Ok(1) => continue,                                                           // blank line
			Ok(len) => println!("{} {}", Local::now().format(&format), &buf[..len - 1]), // strip trailing newline
			Err(e) => panic!("Error writing: {:?}", e),
		}
		buf.clear();
	}
}
