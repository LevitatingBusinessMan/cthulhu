#![feature(once_cell)]
use irc::client::prelude::*;
use anyhow::{Result, Error};
use futures::prelude::*;
use crate::logger::log;

//static PREFIX: &'static str = "!";

#[macro_use]
pub mod commands;
pub mod user;
pub mod config;
#[macro_use]
pub mod color;
pub mod disponse;
#[macro_use]
pub mod logger;
pub mod client;

use client::client;
use user::User;

#[tokio::main]
/// Welcome to r'lyeh
async fn main() -> Result<(), Error> {
    //https://docs.rs/irc/0.15.0/irc/client/data/config/struct.Config.html
    let config = config::CONFIG.clone();
    let prefix = config::PREFIX.to_string();
    linfo!("Using prefix: {}", prefix);

    let mut stream = client::init(config).await?;
    let sender = client().sender();

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);

        /*
        Run handler depending on command type
        */

        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.starts_with(&prefix) {
                let mut argv = msg.split_ascii_whitespace();
                let command = argv.next().unwrap()[prefix.len()..].to_owned();
                let arguments = argv.map(|x| x.to_owned()).collect::<Vec<String>>();
                let target = target.to_owned();
                handle_command(message, command, arguments, target, &sender).await?;
            }
        }
    }

    Ok(())
}

/*
Hashmap with functions.
Sled
*/
async fn handle_command(
    message: Message,
    command: String,
    arguments: Vec<String>,
    target: String,
    sender: &Sender
) -> anyhow::Result<(),anyhow::Error> {
    if let Some(cmd) = commands::get_command(command.as_ref()) {

        let user = User::try_from(&message)?;

        if let Err(error) = cmd.check(&message, &user, &arguments) {
            let error_string = error.to_string();
            sender.send_privmsg(message.response_target().unwrap_or(&target),format!("Error: {}", error_string))?;
            return Ok(())
        }

        let result = cmd.run(user, arguments, &target);
        sender.send_privmsg(message.response_target().unwrap_or(&target),result)?;
        return Ok(())

    } else {
        Ok(match disponse::get(&command) {
            Some(dispo) => {
                let user = User::try_from(&message)?;
                sender.send_privmsg(
                    message.response_target().unwrap_or(&target),
                    disponse::replace_specials(dispo.text, arguments.join(" "), message.response_target().unwrap_or(&target), user)
                )?
            },
            None => ()
        })
    }
}
