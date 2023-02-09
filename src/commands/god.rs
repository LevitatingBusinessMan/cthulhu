use crate::commands::*;

pub struct God;

impl CommandDetails for God {
	const ARITY: Arity = Arity::Exact(0);
	const NAME: &'static str = "god";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec![]);
	const GOD: bool = false;
	const USAGE: &'static str = "";
	const DESCRIPTION: &'static str = "Let's you know if you're a god.";
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
	crate::command_methods!();
}
