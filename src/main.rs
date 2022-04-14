use std::{
	env,
	io::{self, Write},
	process,
};

use chrono::Local;
fn main() {
	ctrlc::set_handler(|| {
		io::stdout().flush().expect("Error flushing stdout");
		process::exit(0);
	})
	.expect("Error setting exit handler!");

	let mut args = env::args();
	let _ = args.next(); // program name
	let format = args.next().unwrap_or("[%H:%M:%S]".to_string());

	let mut buf = String::new();
	let stdin = io::stdin();

	while let Ok(len) = stdin.read_line(&mut buf) {
		let now = Local::now();
		if len > 1 {
			println!("{} {}", now.format(&format), &buf[..len - 1]);
		}
		buf.clear();
	}
}
