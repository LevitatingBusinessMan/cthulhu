use crate::commands::*;

pub struct Ping;

impl CommandDetails for Ping {
	const Arity: Arity = Arity::Exact(1);
	const Aliases: Vec<&'static str> = vec![];
	const God: bool = false;
}

impl CommandMethods for Ping {
	fn run(&self, arguments: Vec<String>, _target: &String) -> String {
		return "pong".to_owned();
	}
	fn aliases(&self) -> Vec<&'static str> {
		Self::Aliases
	}
	fn arity(&self) -> Arity {
		Self::Arity
	}
	fn god(&self) -> bool {
		Self::God
	}
}
