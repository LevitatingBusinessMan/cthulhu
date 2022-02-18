//! This module contains all commands, their generic traits, and maps them together as trait objects

use std::collections::HashMap;
use lazy_static::lazy_static;

pub mod ping;
pub mod god;
pub mod help;

pub mod errors;

pub enum Arity {
	Minimum(u8),
	Exact(u8),
}

use irc::client::prelude::Message;
use crate::user::User;


lazy_static! {
	pub static ref COMMAND_MAP: HashMap::<&'static str, Box<dyn CommandMethods + Sync>> = {
		let mut map = HashMap::new();
		register(&mut map);
		map
	};
}

/// Register all enabled commands to the command map
fn register(map: &mut HashMap<&'static str, Box<dyn CommandMethods + Sync>>) {
	map.insert("ping", Box::new(ping::Ping));
	map.insert("god", Box::new(god::God));
	map.insert("help", Box::new(help::Help));
}


/// Attempt to fetch a command
pub fn get_command(command: &str) -> Option<&Box<dyn CommandMethods + Sync>> {
	COMMAND_MAP.get(command)
}

/// Main interface for commands
pub trait CommandMethods {
	fn run(&self, user: User, arguments: Vec<String>, target: &String) -> String;
	fn name(&self) -> &'static str;
	fn arity(&self) -> Arity;
	fn aliases(&self) -> Vec<&'static str>;
	fn god(&self) -> bool;
	fn usage(&self) -> &'static str;
	fn description(&self) -> &'static str;
	fn check(&self, _message: &Message, arguments: &Vec<String>) -> Result<(),errors::Error> {
		let argc = arguments.len();
		let arity_type = self.arity();
		let arity_error = match arity_type {
			Arity::Exact(n) => {
				if n != argc as u8 {
					Some(errors::ArityError {
						arity_type: arity_type,
						actual: argc as u8
					})
				} else {
					None
				}
			},
			Arity::Minimum(n) => {
				if n > argc as u8 {
					Some(errors::ArityError {
						arity_type: arity_type,
						actual: argc as u8
					})
				} else {
					None
				}
			}
		};

		if let Some(a_err) = arity_error {
			return Err(errors::Error::ArityError(a_err))
		}

		return Ok(())
	}
	fn help(&self) -> String {
		if self.usage().is_empty() {
			format!("{} - {}", self.name(), self.description())
		} else {
			format!("{} {} - {}", self.name(), usage_color(self.usage()), self.description())
		}
	}
}

/// Colorize a usage string
fn usage_color(usage_string: &'static str) -> String {
	// Optional arg
	usage_string.to_owned().replace("<", "\x0314<").replace(">", ">\x0315")
}

/// A collection of constants for commands.
/// These can't be part of the CommandMethods struct of object safety.
/// CommandMethods has appropiate methods to access these.
/// https://doc.rust-lang.org/reference/items/traits.html#object-safety
pub trait CommandDetails {
	const ARITY: Arity;
	const NAME: &'static str;
	const ALIASES: Vec<&'static str>;
	const GOD: bool;
	const USAGE: &'static str;
	const DESCRIPTION: &'static str;
}
