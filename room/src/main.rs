extern crate ___bot___;
extern crate botwrapper;

use std::env;

use botwrapper::run;

fn main() {
	let args = env::args().collect();
	run::<___bot___::___Bot___>(args);
}
