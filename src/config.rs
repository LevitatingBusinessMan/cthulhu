//! This module loads the config, and interfaces with its options

use irc::client::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::load("config.toml").unwrap();
}

pub fn get_gods() -> Vec<String> {
	CONFIG.get_option("gods").unwrap().clone().try_into().unwrap()
}

pub fn is_god(hostname: &String) -> bool {
	//hostname == "user/levitating"
	for god in get_gods() {
		if &god == hostname {
			return true;
		}
	}
	return false
}
