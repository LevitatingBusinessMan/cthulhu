use crate::commands::*;

pub struct Delete;

impl CommandDetails for Delete {
	const ARITY: Arity = Arity::Exact(1);
	const NAME: &'static str = "delete";
	const ALIASES: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec!["remove"]);
	const GOD: bool = false;
	const USAGE: &'static str = "<name>";
	const DESCRIPTION: &'static str = "Delete a disponse";
}

use crate::disponse;

impl CommandMethods for Delete {
	fn run(&self, user: User, arguments: Vec<String>, _target: &String) -> String {
		let name = &arguments.first().unwrap().to_lowercase();
		if !user.god {
			match disponse::get(name) {
				None => format!("Disponse {} does not exist.", name),
				Some(dispone) => {
					if dispone.user.hostname == user.hostname {
						match disponse::remove(name) {
							Ok(_) => format!("Disponse {} deleted.", name),
							Err(_) => "Something went wrong accessing the database".to_owned()
						}
					} else {
						"You are not the author of this disponse".to_owned()
					}
				}
			}
			
		} else {
			match disponse::exists(name) {
				Ok(true) => match disponse::remove(name) {
					Ok(_) => format!("Disponse {} deleted.", name),
					Err(_) => "Something went wrong accessing the database".to_owned()
				},
				Ok(false) => return format!("Disponse {} does not exist.", name),
				Err(_) => return "Something went wrong accessing the database".to_owned()
			}
		}
	}
	crate::command_methods!();
}
