extern crate bot;
use bot::libsrl;
pub extern crate time;
mod fs;
mod proof;
mod room;
use std::env;

fn main() {
	let args : Vec<String> = env::args().collect();
	if args[1] == "new" {
		let ref instancepath = args[2];
		room::new(instancepath);
	} else if args[1] == "exec" {
		let ref instancepath = args[2];
		let ref proofspath = args[3];
		room::exec(instancepath, proofspath);
	} else {
		println!("unknown command");
	}
}
