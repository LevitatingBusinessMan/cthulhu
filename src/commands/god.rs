use crate::commands::*;

pub struct God;

impl CommandDetails for God {
	const ARITY: Arity = Arity::Exact(0);
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
}

use crate::{red, green};

impl CommandMethods for God {
	fn run(&self, user: User, _arguments: Vec<String>, _target: &String) -> String {
		if user.god {
			green!((user.god))
		} else{
			red!((user.god))
		}
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
