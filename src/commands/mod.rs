//! This module contains all commands, their generic traits, and maps them together as trait objects

use std::collections::HashMap;
use lazy_static::lazy_static;
use std::cell::LazyCell;

pub mod ping;
pub mod god;
pub mod help;
pub mod save;
pub mod type_;
pub mod choose;
pub mod join;
pub mod dispoinfo;
pub mod delete;

pub mod errors;

pub enum Arity {
	Minimum(u8),
	Exact(u8),
}

use irc::client::prelude::Message;
use crate::user::User;
use crate::bold;


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
	map.insert("save", Box::new(save::Save));
	map.insert("type", Box::new(type_::Type));
	map.insert("choose", Box::new(choose::Choose));
	map.insert("join", Box::new(join::Join));
	map.insert("dispoinfo", Box::new(dispoinfo::DispoInfo));
	map.insert("delete", Box::new(delete::Delete));
}

#[macro_export]
/// The [`CommandMethods`] trait requires a bunch of getter methods for constants defined in the [`CommandDetails`] trait.
/// However [`CommandMethods`] that refer to constants defined in the [`CommandDetails`] trait can't be defined as default methods.
/// To avoid having to copy paste these methods everywhere this macro is used instead
macro_rules! command_methods {
	() => {
		fn aliases(&self) -> Vec<&'static str> {
			std::cell::LazyCell::<Vec<&'static str>>::force(&Self::ALIASES).clone()
		}
		fn name(&self) -> &'static str {
			Self::NAME
		}
		fn arity(&self) -> Arity {
			Self::ARITY
		}
		fn god(&self) -> bool {
			Self::GOD
		}
		fn usage(&self) -> &'static str {
			Self::USAGE
		}
		fn description(&self) -> &'static str {
			Self::DESCRIPTION
		}
	};
}

/// Attempt to fetch a command
pub fn get_command(command: &str) -> Option<&Box<dyn CommandMethods + Sync>> {
	match COMMAND_MAP.get(command) {
		Some(cmd) => Some(cmd),
		None => COMMAND_MAP.values().find(|c| c.aliases().contains(&command)),
	}
}

/// Main interface for commands
pub trait CommandMethods {
	fn run(&self, user: User, arguments: Vec<String>, target: &String) -> String;
	/// currently unused
	fn name(&self) -> &'static str;
	fn arity(&self) -> Arity;
	/// currently unused
	fn aliases(&self) -> Vec<&'static str>;
	fn god(&self) -> bool;
	fn usage(&self) -> &'static str;
	fn description(&self) -> &'static str;
	fn check(&self, _message: &Message, user: &User, arguments: &Vec<String>) -> Result<(),errors::Error> {
		if let Err(arity_error) = arity_check(arguments.len() as u8, self.arity()) {
			return Err(errors::Error::ArityError(arity_error));
		}

		if self.god() && !user.god {
			return Err(errors::Error::PermissionError(anyhow::anyhow!("You don't have the permission to run this command.")));
		}

		Ok(())
	}
	fn help(&self) -> String {
		if self.usage().is_empty() {
			format!("{} - {}", bold!(self.name()), self.description())
		} else {
			format!("{} {} - {}", bold!(self.name()), usage_color(self.usage()), self.description())
		}
	}
}

/// Generates a help page
/// If the user is god then it will include god commands
fn generate_help(has_god: bool) -> String {
	let mut command_list = vec![];
	for (_name, cmd) in COMMAND_MAP.iter() {
		if has_god  || !cmd.god() {
			command_list.push(cmd.name()) 
		}
	}
	format!("Welcome to r'lyeh, where cthulhu awaits dead but sleeping. Available commands: {}", command_list.join(", "))
}

/// Check if the right amount of arguments were used
fn arity_check(arcg: u8, arity_type: Arity) -> Result<(),errors::ArityError> {
	let arity_error = match arity_type {
		Arity::Exact(n) => {
			if n != arcg {
				Some(errors::ArityError {
					arity_type: arity_type,
					actual: arcg
				})
			} else {
				None
			}
		},
		Arity::Minimum(n) => {
			if n > arcg {
				Some(errors::ArityError {
					arity_type: arity_type,
					actual: arcg
				})
			} else {
				None
			}
		}
	};

	if let Some(a_err) = arity_error {
		return Err(a_err)
	}

	Ok(())
}

/// Colorize a usage string
fn usage_color(usage_string: &'static str) -> String {
	// Optional arg
	usage_string.to_owned()
		.replace("[", "\x0314[").replace("]", "]\x03")
		.replace("<", "\x1d<").replace(">", ">\x1d")
}

/// A collection of constants for commands.
/// These can't be part of the CommandMethods struct due to [object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety).
/// CommandMethods has appropiate methods to access these.
pub trait CommandDetails {
	const ARITY: Arity;
	const NAME: &'static str;
	/// A list of aliases for the command
	const ALIASES: LazyCell<Vec<&'static str>>;
	/// If the command can only be run by an admin
	const GOD: bool;
	/// The usage string, use \<arg\> to denote required arguments and \[arg\] for optional arguments
	const USAGE: &'static str;
	/// A short description of the command
	const DESCRIPTION: &'static str;
}
