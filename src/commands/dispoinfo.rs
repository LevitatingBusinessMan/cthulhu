use crate::commands::*;
use crate::disponse;
pub struct DispoInfo;

impl CommandDetails for DispoInfo {
	const ARITY: Arity = Arity::Exact(1);
	const NAME: &'static str = "dispoinfo";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec!["dinfo"]);
	const GOD: bool = false;
	const USAGE: &'static str = "<disponse>";
	const DESCRIPTION: &'static str = "Get the author and creation time of a disponse";
}

impl CommandMethods for DispoInfo {
	fn run(&self, _user: User, arguments: Vec<String>, _target: &String) -> String {
		let name = arguments[0].to_lowercase();
		match disponse::exists(&name) {
			Ok(true) => match disponse::get(&name) {
				Some(disponse) => format!(
					"{}: Created by {}@{} at {}",
					disponse.name,
					disponse.user.nickname,
					disponse.user.hostname,
					chrono::DateTime::<chrono::Utc>::from(disponse.timestamp).to_rfc2822()
				),
				None => "Something went wrong accessing the database".to_owned()
			},
			Ok(false) => "Disponse does not exist".to_owned(),
			Err(_) => "Something went wrong accessing the database".to_owned()
		}
	}
	crate::command_methods!();
}
