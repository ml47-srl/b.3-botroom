use std::path::Path;
use std::fs::{remove_dir, create_dir, remove_file, read_dir, ReadDir, File};
use std::io::{Write, Read};
use libsrl::error::SRLError;

pub fn assert_dir(path : &Path) {
	if path.is_file() {
		remove_file(path).unwrap();
	}
	if !path.exists() {
		create_dir(path).unwrap();
	}
}

pub fn assert_file(path : &Path) {
	if path.is_dir() {
		remove_dir(path).unwrap();
	}
	if !path.exists() {
		File::create(path).unwrap();
	}
}

pub fn ls(path : &Path) -> Result<Vec<String>, SRLError> {
	let rd : ReadDir = match read_dir(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("ls".to_string(), "error opening dir".to_string()))
	};
	let mut vec : Vec<String> = Vec::new();
	for entry in rd {
		if let Ok(x) = entry {
			let os_str = x.file_name();
			let filename = match os_str.to_str() {
				Some(x) => x,
				None => return Err(SRLError("ls".to_string(), "could not convert OsStr to &str".to_string()))
			};
			vec.push(filename.to_string());
		}
	}
	Ok(vec)
}

pub fn force_file(path : &Path, content : &str) -> Result<(), SRLError> {
	assert_file(path);
	write_file(path, content)
}

pub fn write_file(path : &Path, content : &str) -> Result<(), SRLError> {
	let mut file : File = match File::create(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("write_file".to_string(), "error opening file".to_string()))
	};
	match file.write(content.as_bytes()) {
		Ok(_) => Ok(()),
		Err(_) => Err(SRLError("write_file".to_string(), "error writing file".to_string()))
	}
}

pub fn read_file(path : &Path) -> Result<String, SRLError> {
	let mut file : File = match File::open(path) {
		Ok(x) => x,
		Err(_) => return Err(SRLError("read_file".to_string(), "error opening file".to_string()))
	};
	let mut string = String::new();
	match file.read_to_string(&mut string) {
		Ok(_) => Ok(string),
		Err(_) => Err(SRLError("read_file".to_string(), "error reading file".to_string()))
	}
}
