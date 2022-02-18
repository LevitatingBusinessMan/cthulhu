use crate::commands::*;

pub struct Ping;

impl CommandDetails for Ping {
	const ARITY: Arity = Arity::Exact(0);
	const NAME: &'static str = "ping";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = true;
	const USAGE: &'static str = "";
	const DESCRIPTION: &'static str = "Responds with pong.";
}

impl CommandMethods for Ping {
	fn run(&self, _user: User, _arguments: Vec<String>, _target: &String) -> String {
		return "pong".to_owned();
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
