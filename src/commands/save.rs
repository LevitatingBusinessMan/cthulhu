pub struct Save;

impl crate::commands::Command for Ping {
	fn run(&self, arguments: Vec<String>, _target: &String) -> String {
		return "pong".to_owned();
	}
}
