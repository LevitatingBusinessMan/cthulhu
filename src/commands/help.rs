use crate::commands::*;
use std::cell::LazyCell;

pub struct Help;

impl CommandDetails for Help {
	const ARITY: Arity = Arity::Minimum(0);
	const NAME: &'static str = "help";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec![]);
	const GOD: bool = false;
	const USAGE: &'static str = "[command]";
	const DESCRIPTION: &'static str = "Either shows a general help page or a help page for a specific command.";
}

impl CommandMethods for Help {
	fn run(&self, user: User, arguments: Vec<String>, _target: &String) -> String {
		if arguments.len() == 0 {
			generate_help(user.god)
		} else {
			let command = &arguments[0];
			get_command(command).map(|c| c.help()).unwrap_or_else(|| "There's no such command.".to_owned())
		}
	}
	crate::command_methods!();
}
