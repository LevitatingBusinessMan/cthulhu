//! This module owns the client
use irc::client::{prelude::*, ClientStream};
use std::sync::OnceLock;

pub static CLIENT: OnceLock<Client> = OnceLock::new();

/// Initialize the client and receive the [`ClientStream`]
pub async fn init(config: Config) -> Result<ClientStream, irc::error::Error> {
	let mut client = Client::from_config(config).await?;
	client.identify()?;
	let stream = client.stream()?;
    CLIENT.set(client).unwrap();
	Ok(stream)
}

/// Getter function for the static [`Client`]
pub fn client() -> &'static Client {
	CLIENT.get().unwrap()
}
