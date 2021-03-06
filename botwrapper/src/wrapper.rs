use std::path::Path;

use time::now;
use proof::Proof;
use fs::{read_file, write_file, force_file};
use botfather::libsrl::db::Database;
use botfather::libsrl::cell::Cell;
use botfather::{StopReason, Botfather};
use std::time::Duration;

fn exec<Bot: Botfather>(instancepath_str : &str, proofspath_str : &str) {
	let proofs = get_proofs(Path::new(proofspath_str));

	let instancepath = Path::new(instancepath_str);
	let botfile_pbuf = instancepath.join("botfile");

	let content = read_file(botfile_pbuf.as_path());
	let mut bot : Bot = Bot::by_string(&content.unwrap());

	let mut result : String = String::new();

	for i in 0..proofs.len() {
		let proof : &Proof = &proofs[i];
		let (stop_reason, time, tmp_bot) = exec_single(bot, proof);
		bot = tmp_bot;
		result.push_str(&get_result_line(i, stop_reason, time));
	}
	write_file(botfile_pbuf.as_path(), &bot.to_string()).unwrap();
	let id = get_free_result_id(instancepath);
	let pbuf = instancepath.join("r".to_string() + &id.to_string());
	force_file(pbuf.as_path(), &result).unwrap();
}

fn new<Bot: Botfather>(instancepath : &str) {
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

fn exec_single<Bot: Botfather>(bot : Bot, proof : &Proof) -> (StopReason, u32, Bot) {
	use std::thread;
	use std::sync::mpsc;
	use std::mem::drop;

	let (s, r) = mpsc::channel();

	let src_db : Database = (*proof.get_db()).clone();
	let src_target : Cell = (*proof.get_target()).clone();

	let mut th_db : Database = src_db.clone();
	let th_target : Cell = src_target.clone();
	let th_bot = bot.clone();

	let timeout = Duration::from_secs(5); // TODO dynamic timeout
	let start_time = now().to_timespec();

	let th1_s = s.clone();
	let th1 = thread::spawn(move || {
		th_bot.call(&mut th_db, &th_target);
		th1_s.send(Some((th_bot, th_db))).unwrap();
	});

	let th2_s = s.clone();
	let th2 = thread::spawn(move || {
		thread::sleep(timeout);
		th2_s.send(None).unwrap();
	});

	let result_option = r.recv().unwrap();

	let time : u32 = (now().to_timespec() - start_time).num_milliseconds() as u32;

	drop(th1);
	drop(th2);

	let wanted_result : Vec<Cell> = { let mut x = src_db.get_rules().clone(); x.push(src_target); x };

	if let Some((out_bot, out_db)) = result_option {
		let stop_reason = if out_db.get_rules() == wanted_result { StopReason::Win } else { StopReason::Fail };

		return (stop_reason, time, out_bot);
	} else  {
		return (StopReason::Timeout, time, bot);
	}
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
	return i;
}

pub fn run<Bot: Botfather>(args: Vec<String>) {
	if args[1] == "new" {
		let ref instancepath = args[2];
		new::<Bot>(instancepath);
	} else if args[1] == "exec" {
		let ref instancepath = args[2];
		let ref proofspath = args[3];
		exec::<Bot>(instancepath, proofspath);
	} else {
		println!("unknown command");
	}
}
