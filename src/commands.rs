use crate::plugin::PluginManager;
use std::io;
use std::path::Path;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Command {
    Load(String),
    Remove(String),
    List,
}

pub fn user_input(tx: mpsc::UnboundedSender<Command>) -> ! {
    let handle = io::stdin();

    loop {
        let mut buffer = String::new();
        handle.read_line(&mut buffer).unwrap();

        let mut iter = buffer.split_ascii_whitespace();

        let command: Option<Command> = match iter.next() {
            Some(x) => match x {
                "load" => Some(Command::Load(iter.next().unwrap().to_string())),
                "remove" => Some(Command::Remove(iter.next().unwrap().to_string())),
                "list" => Some(Command::List),
                _ => None,
            },
            None => None,
        };

        let Some(cmd) = command else {
            continue;
        };

        tx.send(cmd).unwrap();
    }
}

pub fn exe(cmd: Command, manager: &mut PluginManager) {
    match cmd {
        Command::List => {
            for x in manager.plugins() {
                println!("{}", x);
            }
        }
        Command::Load(s) => manager.register(Path::new(&s)).unwrap(),
        Command::Remove(s) => (),
    }
}
