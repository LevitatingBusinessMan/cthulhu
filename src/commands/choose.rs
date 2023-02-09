use rand::seq::SliceRandom;
use crate::commands::*;

pub struct Choose;

impl CommandDetails for Choose {
	const ARITY: Arity = Arity::Minimum(1);
	const NAME: &'static str = "choose";
	const USAGE: &'static str = "cheese; bacon [; more options]";
	const DESCRIPTION: &'static str = "Choose between any amount of options (dividied by ';')";
}

impl CommandMethods for Choose {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		arguments.join(" ").split(';').collect::<Vec<&str>>().choose(&mut rand::thread_rng()).unwrap().trim().to_string()
	}
	crate::command_methods!();
}
