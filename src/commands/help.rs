use crate::commands::*;

pub struct Help;

impl CommandDetails for Help {
	const ARITY: Arity = Arity::Minimum(0);
	const NAME: &'static str = "help";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
	const USAGE: &'static str = "<command>";
	const DESCRIPTION: &'static str = "Either shows a general help page or a help page for a specific command.";
}

impl CommandMethods for Help {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		if arguments.len() == 0 {
			return "help".to_owned();
		} else {
			let command = &arguments[0];
			get_command(command).map(|c| c.help()).unwrap_or_else(|| "There's no such command.".to_owned())
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
