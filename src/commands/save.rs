use crate::commands::*;

pub struct Save;

impl CommandDetails for Save {
	const ARITY: Arity = Arity::Minimum(2);
	const NAME: &'static str = "save";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
	const USAGE: &'static str = "name text";
	const DESCRIPTION: &'static str = "Saves a response to the disponses table. (basically factoids)";
}

use crate::disponse;

impl CommandMethods for Save {
	fn run(&self, user: User, arguments: Vec<String>, _target: &String) -> String {
		let name = &arguments[0];
		let text = arguments[1..].join(" ");
		if user.god {
			match disponse::save(&user, name, &text) {
				Ok(_) => "Succesfully saved disponse".to_owned(),
				Err(_) => "Something went wrong accessing the database".to_owned()
			}
		} else {
			"This command is currently not available for unprivileged users".to_string()
		}
	}
	crate::command_methods!();
}
