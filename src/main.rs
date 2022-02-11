use irc::client::prelude::*;
use anyhow::{Result, Error};
use futures::prelude::*;
use std::collections::HashMap;

static PREFIX: &'static str = "!";

pub mod commands;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // We can also load the Config at runtime via Config::load("path/to/config.toml")
    let config = Config {
        nickname: Some("cthulhu".to_owned()),
        server: Some("irc.libera.chat".to_owned()),
        channels: vec!["##test".to_owned()],
        ..Config::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();


    let mut commandsMap = HashMap::<&'static str, Box<dyn commands::Command>>::new();
    commands::register(&mut commandsMap);

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);

        /*
        Run handler depending on command type
        */

        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.starts_with(PREFIX) {
                let mut argv = msg.split_ascii_whitespace();
                let command = argv.next().unwrap()[1..].to_owned();
                let arguments = argv.map(|x| x.to_owned()).collect::<Vec<String>>();
                let target = target.to_owned();
                println!("{}: {} {}", target, command, arguments.join(" "));
                handle_command(&mut commandsMap, command, arguments, target, &sender).await?;
            }
        }
    }

    Ok(())
}

/*
Hashmap with functions.
Sled
*/
async fn handle_command(map: &mut HashMap::<&'static str, Box<dyn commands::Command>>, command: String, arguments: Vec<String>, target: String, sender: &Sender) -> Result<(),irc::error::Error> {
    if let Some(cmd) = map.get::<str>(command.as_ref()) {
        let result = cmd.run(arguments, &target);
        return sender.send_privmsg(&target,result);
    } else {
        //SLED
        return Ok(())
    }
}
