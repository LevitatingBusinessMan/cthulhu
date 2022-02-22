use crate::commands::*;
use crate::disponse;

pub struct Type;

impl CommandDetails for Type {
	const ARITY: Arity = Arity::Exact(1);
	const NAME: &'static str = "type";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
	const USAGE: &'static str = "command";
	const DESCRIPTION: &'static str = "Check if a command is an actual command or a disponse";
}

impl CommandMethods for Type {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		let cmd = &arguments[0];
		if get_command(cmd).is_some() {
			format!("{} is a command", cmd)
		} else if disponse::get(cmd).is_some() {
			format!("{} is a disponse", cmd)
		} else {
			format!("{} is not a command or disponse", cmd)
		}
	}
	fn aliases(&self) -> Vec<&'static str> {
		Self::ALIASES
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
}
