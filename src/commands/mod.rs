use std::collections::HashMap;

pub mod ping;

pub trait Command {
	fn run(&self, arguments: Vec<String>, target: &String) -> String;
}

pub fn register(map: &mut HashMap<&'static str, Box<dyn Command>>) {
	map.insert("ping", Box::new(ping::Ping));
}
