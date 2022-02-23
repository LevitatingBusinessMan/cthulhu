//! This module loads the config, and interfaces with its options

use irc::client::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::load("config.toml").unwrap();
	pub static ref PREFIX: String = CONFIG.options.get("prefix").unwrap().as_str().unwrap().to_owned();
	pub static ref GODS: Vec<String> = CONFIG.get_option("gods").unwrap().clone().try_into().unwrap();
}

pub fn get_gods<'a>() -> &'a Vec<String> {
	// This has yet to use the static
	&GODS
}

pub fn is_god(hostname: &String) -> bool {
	//hostname == "user/levitating"
	for god in get_gods() {
		if &god == &hostname {
			return true;
		}
	}
	return false
}
