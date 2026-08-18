use crate::plugin::{Request, Response};
use std::io::{self, BufRead};
use tokio::sync::{mpsc, oneshot};

/// Commands
/// List [clients | plugins]
/// Scan
/// Load [path]
/// Unload [plugin name]

/*fn parse_input(buf: &str) -> Option<Request> {
    let mut iter = buf.trim().split_whitespace().array_chunks::<2>();

    match command {
        ("load", a) => {}
        ("list", a) => {}
        ("scan", a) => {}
    }
}*/

/// Does not return as this should be running on other thread for the duration of the program
pub fn user_input(plg_tx: mpsc::UnboundedSender<(Request, oneshot::Sender<Response>)>) -> ! {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut buf = String::new();

        match reader.read_line(&mut buf) {
            Ok(0) => std::process::exit(0),
            Ok(_) => {} // There is data to read
            Err(e) => {
                println!("Error reading: {}", e);
                continue;
            }
        }

        let trimmed = buf.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut iter = trimmed.split_ascii_whitespace();

        let command = match iter.next() {
            Some("load") => {
                let dir = match iter.next() {
                    Some(arg) => arg.to_string().into(),
                    None => {
                        println!("no path");
                        continue;
                    }
                };
                Request::Register { dir }
            }
            Some("remove") => {
                let plugin = match iter.next() {
                    Some(arg) => arg.to_string(),
                    None => {
                        println!("need plugin name");
                        continue;
                    }
                };
                Request::Deregister { plugin }
            }
            Some("list") => Request::List,
            Some(other) => {
                println!("Unknown command: {}", other);
                continue;
            }
            None => continue,
        };

        // Sends the user input to the plugin manager thread
        let (tx, rx) = oneshot::channel();
        if let Err(e) = plg_tx.send((command, tx)) {
            println!("Error: Couldn't send command {}", e);
            continue;
        }

        // Waits for the request to be processed
        match rx.blocking_recv() {
            Ok(Response::List(m)) => println!("{}", m),
            Ok(Response::Success) => println!("Operation finished!"),

            Ok(Response::Failed(m)) => println!("Error: {:?}", m),
            Err(e) => println!("Error: {}", e),
            _ => print!("Unahndled"),
        }
    }
}
