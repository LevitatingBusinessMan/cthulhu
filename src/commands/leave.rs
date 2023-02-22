use crate::commands::*;

pub struct Leave;

impl CommandDetails for Leave {
	const ARITY: Arity = Arity::Minimum(0);
	const NAME: &'static str = "leave";
	const GOD: bool = true;
	const USAGE: &'static str = "<channel1> [channel2]";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec!["part"]);
	const DESCRIPTION: &'static str = "Part a channel.";
}

impl CommandMethods for Leave {
	fn run(&self, _user: User, arguments: Vec<String>, target: &String) -> String {
		let channels = if arguments.len() > 0 {arguments.join(",")} else {target.clone()};
		match crate::client::client().send_part(channels) {
			Ok(_) => "Part command send".to_owned(),
			Err(err) => format!("Something went wrong parting the channel: {}",err).to_owned()
		}
	}
	crate::command_methods!();
}
