use crate::commands::*;

pub struct Ping;

impl CommandDetails for Ping {
	const ARITY: Arity = Arity::Exact(0);
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
}

impl CommandMethods for Ping {
	fn run(&self, _user: User, _arguments: Vec<String>, _target: &String) -> String {
		return "pong".to_owned();
	}
	fn aliases(&self) -> Vec<&'static str> {
		Self::ALIASES
	}
	fn arity(&self) -> Arity {
		Self::ARITY
	}
	fn god(&self) -> bool {
		Self::GOD
	}
}
