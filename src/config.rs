//! This module loads the config, and interfaces with its options

use irc::client::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::load("config.toml").unwrap();
	pub static ref PREFIX: &'static str = CONFIG.options.get("prefix").unwrap().as_str().unwrap();
	pub static ref GODS: Vec<&'static str> = CONFIG.get_option("gods").unwrap().as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
}

pub fn get_gods<'a>() -> &'a Vec<&'static str> {
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
