use libsrl::cell::Cell;
use libsrl::db::Database;
use libsrl::error::SRLError;
use fs::{assert_dir, force_file, read_file};
use std::path::Path;

pub struct Proof {
	target : Cell,
	db : Database
}

impl Proof {
	pub fn create(target : Cell, db : Database) -> Proof {
		Proof { target : target, db : db }
	}

	pub fn get_target(&self) -> &Cell {
		&self.target
	}

	pub fn get_db(&self) -> &Database {
		&self.db
	}

	pub fn to_dir(&self, path : &Path) -> Result<(), SRLError> {
		assert_dir(path);

		let target_pbuf = path.join("target");
		force_file(target_pbuf.as_path(), &self.target.to_string()).unwrap();

		let db_pbuf = path.join("db");
		force_file(db_pbuf.as_path(), &self.db.to_string()).unwrap();

		Ok(())
	}

	pub fn from_dir(path : &Path) -> Result<Proof, SRLError> {
		let target_pbuf = path.join("target");
		let target_string = read_file(target_pbuf.as_path())?;
		let target = Cell::by_string(&target_string)?;

		let db_pbuf = path.join("db");
		let db_string = read_file(db_pbuf.as_path())?;
		let db = Database::by_string(&db_string)?;

		Ok(Proof { target : target, db : db })
	}
}
