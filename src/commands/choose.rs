use rand::seq::SliceRandom;
use crate::commands::*;

pub struct Choose;

impl CommandDetails for Choose {
	const ARITY: Arity = Arity::Minimum(1);
	const NAME: &'static str = "choose";
	const ALIASES: Vec<&'static str> = vec![];
	const GOD: bool = false;
	const USAGE: &'static str = "cheese; bacon (; more options)";
	const DESCRIPTION: &'static str = "Choose between any amount of options (dividied by ';')";
}

impl CommandMethods for Choose {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		arguments.join(" ").split(';').collect::<Vec<&str>>().choose(&mut rand::thread_rng()).unwrap().trim().to_string()
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
