use std::collections::HashMap;

pub mod ping;
pub mod errors;

pub enum Arity {
	Minimum(u8),
	Exact(u8),
}

use irc::client::prelude::Message;

pub trait CommandMethods {
	fn run(&self, arguments: Vec<String>, target: &String) -> String;
	fn arity(&self) -> Arity;
	fn aliases(&self) -> Vec<&'static str>;
	fn god(&self) -> bool;
	fn check(&self, message: &Message, arguments: &Vec<String>) -> Result<(),errors::Error> {
		let argc = arguments.len();
		let arity_type = self.arity();
		let arity_error = match arity_type {
			Arity::Exact(n) => {
				if n != argc as u8 {
					Some(errors::ArityError {
						arity_type: arity_type,
						actual: argc as u8
					})
				} else {
					None
				}
			},
			Arity::Minimum(n) => {
				if n > argc as u8 {
					Some(errors::ArityError {
						arity_type: arity_type,
						actual: argc as u8
					})
				} else {
					None
				}
			}
		};

		if let Some(a_err) = arity_error {
			return Err(errors::Error::ArityError(a_err))
		}

		return Ok(())
	}
}

pub trait CommandDetails {
	const Arity: Arity;
	const Aliases: Vec<&'static str>;
	const God: bool;
}

pub fn register(map: &mut HashMap<&'static str, Box<dyn CommandMethods>>) {
	map.insert("ping", Box::new(ping::Ping));
}
