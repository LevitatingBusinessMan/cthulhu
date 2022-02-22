use crate::commands::*;

pub struct Ping;

impl CommandDetails for Ping {
	const ARITY: Arity = Arity::Exact(0);
	const NAME: &'static str = "ping";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
	const USAGE: &'static str = "";
	const DESCRIPTION: &'static str = "Responds with pong.";
}

impl CommandMethods for Ping {
	fn run(&self, _user: User, _arguments: Vec<String>, _target: &String) -> String {
		return "pong".to_owned();
	}
	crate::command_methods!();
}
