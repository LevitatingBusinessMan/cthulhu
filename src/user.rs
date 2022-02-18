use std::any;

use anyhow::anyhow;
use irc::client::prelude::{Message, Prefix};

//use irc::irc_proto::{Message, Prefix};

pub struct User {
	/// The current nickname in use by this user
	pub nickname: String,

	/// The host or hostmask
	/// This is used to identify a user with
	pub hostname: String,

	/// This is the system username
	pub username: String,

	/// States if this user has administrator permissions
	pub god: bool,
}

impl TryFrom<&Message> for User {
	type Error = anyhow::Error;

	fn try_from(message: &Message) -> Result<Self, Self::Error> {
		if let Some(prefix) = message.prefix.clone() {
			match prefix {
				Prefix::Nickname(nickname, username, hostname) => {
					Ok(User {
						god: crate::config::is_god(&hostname),
						nickname,
						username,
						hostname,
					})
				}, 
				Prefix::ServerName(_) => Err(anyhow!("Prefix is not of type nickname (servername)"))
			}
		} else {
			Err(anyhow!("No prefix"))
		}
	}
}

