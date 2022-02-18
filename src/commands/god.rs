use crate::commands::*;

pub struct God;

impl CommandDetails for God {
	const Arity: Arity = Arity::Exact(0);
	const Aliases: Vec<&'static str> = vec![];
	const God: bool = false;
}

use crate::{red, green};

impl CommandMethods for God {
	fn run(&self, user: User, arguments: Vec<String>, _target: &String) -> String {
		if user.god {
			green!((user.god))
		} else{
			red!((user.god))
		}
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
