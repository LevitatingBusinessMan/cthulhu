use crate::commands::*;

pub struct Join;

impl CommandDetails for Join {
	const ARITY: Arity = Arity::Minimum(1);
	const NAME: &'static str = "join";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec![]);
	const GOD: bool = true;
	const USAGE: &'static str = "";
	const DESCRIPTION: &'static str = "Responds with pong.";
}

impl CommandMethods for Join {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		match crate::client::client().send_join(arguments.join("")) {
			Ok(_) => "Join command send".to_owned(),
			Err(err) => format!("Something went wrong joining the channel: {}",err).to_owned()
		}
	}
	crate::command_methods!();
}
