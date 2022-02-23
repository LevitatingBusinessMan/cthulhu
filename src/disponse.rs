//! In charge of the disponse database.

use lazy_static::lazy_static;

static DISPONSE_DB_PATH: &'static str = "disponses";

use sled::Db;
use crate::user::User;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use bincode;
use crate::logger;

#[derive(Serialize, Deserialize)]
pub struct Disponse {
	/// The user in the state at which the disponse was made
	pub user: User,
	/// Timestamp at which this disponse was made
	pub timestamp: SystemTime,
	/// The name or key for this disponse
	pub name: String,
	/// The actual disponse text
	pub text: String,
}

lazy_static! {
	pub static ref DISPONSES: Db = sled::open(DISPONSE_DB_PATH).unwrap();
}

pub fn save(user: &User, name: &str, text: &str) -> Result<(), sled::Error> {
	let disponse = Disponse {
		user: user.clone(),
		name: name.to_owned(),
		text: text.to_owned(),
		timestamp: SystemTime::now()
	};
	match DISPONSES.insert(name, bincode::serialize(&disponse).unwrap()) {
		Ok(_) => {
			logger::lsuc(&format!("Saved disponse {}: {}", name, text));
			Ok(())
		},
		Err(err) => {
			logger::lerr(&err.to_string());
			Err(err)
		}
	}
}

pub fn get(key: &str) -> Option<Disponse> {
	let value = DISPONSES.get(key).unwrap();
	if let Some(value) = value {
		let disponse: Disponse = bincode::deserialize(&value).unwrap();
		Some(disponse)
	} else {
		None
	}
}

/// The disponses can have fill-ins.
/// Currently only <argv> and <channel> are supported. 
pub fn replace_specials(string: String, arguments: String, channel: &str) -> String {
	string.replace("<argv>", &arguments).replace("<channel>", channel)
}

pub fn exists(key: &str) -> Result<bool, sled::Error> {
	DISPONSES.contains_key(key)
}
