use std::path::Path;

use time::now;
use bot::Bot;
use proof::Proof;
use fs::{read_file, write_file, force_file};
use bot::libsrl::db::Database;
use bot::libsrl::cell::Cell;
use bot::{StopReason, Botfather};

pub fn exec(instancepath_str : &str, proofspath_str : &str) {
	let proofs = get_proofs(Path::new(proofspath_str));

	let instancepath = Path::new(instancepath_str);
	let botfile_pbuf = instancepath.join("botfile");

	let content = read_file(botfile_pbuf.as_path());
	let mut bot = Bot::by_string(content.unwrap());

	let mut result : String = String::new();

	for i in 0..proofs.len() {
		let proof : &Proof = &proofs[i];
		let (stop_reason, time) = exec_single(&mut bot, proof);
		result.push_str(&get_result_line(i, stop_reason, time));
	}
	write_file(botfile_pbuf.as_path(), &bot.to_string()).unwrap();
	let id = get_free_result_id(instancepath);
	let pbuf = instancepath.join("r".to_string() + &id.to_string());
	force_file(pbuf.as_path(), &result).unwrap();
}

pub fn new(instancepath : &str) {
	let instancepath = Path::new(instancepath);
	let botfile_pbuf = instancepath.join("botfile");
	let content = Bot::gen().to_string();
	force_file(botfile_pbuf.as_path(), &content).unwrap();
}

fn get_proof(proofspath : &Path, i : i32) -> Option<Proof> {
	let pbuf = proofspath.join("p".to_string() + &i.to_string());
	match Proof::from_dir(pbuf.as_path()) {
		Ok(x) => Some(x),
		Err(_) => None
	}
}

fn get_proofs(proofspath : &Path) -> Vec<Proof> {
	let mut i = 0;
	let mut vec = Vec::new();
	loop {
		match get_proof(proofspath, i) {
			Some(x) => vec.push(x),
			None => break
		}
		i += 1;
	}
	vec
}

fn exec_single(bot : &mut Bot, proof : &Proof) -> (StopReason, u32) {
	let src_db : Database = (*proof.get_db()).clone();
	let mut db : Database = src_db.clone();

	let start_time = now().to_timespec();
	bot.call(&mut db, proof.get_target());
	let time : u32 = (now().to_timespec() - start_time).num_milliseconds() as u32;
	// TODO timeout

	let mut wanted_result : Vec<Cell> = src_db.get_rules().clone();
	wanted_result.push(proof.get_target().clone());
	let stop_reason = match db.get_rules() == wanted_result {
		true => StopReason::Win,
		false => StopReason::Fail
	};

	(stop_reason, time)
}

fn get_result_line(proof_id : usize, stop_reason : StopReason, time : u32) -> String {
	let mut string = String::new();
	string.push_str(&proof_id.to_string());
	string.push(' ');
	string.push(match stop_reason {
		StopReason::Win => 'w',
		StopReason::Fail => 'f',
		StopReason::Timeout => 't'
	});
	string.push(' ');
	string.push_str(&time.to_string());
	string.push('\n');
	string
}

fn get_free_result_id(instancepath : &Path) -> u32 {
	let mut i = 0;
	loop {
		let pbuf = instancepath.join("r".to_string() + &i.to_string());
		if pbuf.as_path().exists() {
			i += 1;
		} else {
			break;
		}
	}
	i
}
