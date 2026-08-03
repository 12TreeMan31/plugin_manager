use std::io;
use tokio::sync::mpsc;

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

        let mut iter = buffer.split(' ');

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
