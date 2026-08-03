/*
    Client Connects and opens websocket
    Server waits for client to provide the plugin they would like to use
    Server then checks if plugin is valid. Closes connection if can't, spawns task with plugin if can
    Task ends when the client closes the stream
*/

use futures_util::StreamExt;
use plugin_manager::commands::*;
use plugin_manager::plugin::PluginManager;
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_tungstenite::{WebSocketStream, accept_hdr_async};
use tungstenite::handshake::server::{ErrorResponse, Request, Response};

const DEFAULT_ADDR: &str = "127.0.0.1:8080";

struct Client {
    pub socket: WebSocketStream<TcpStream>,
    pub namespace: String,
}

/// Main event loop for the plugin manager that can:
/// * Manage plugins from the user (load or remove)
/// * Allow clients to join a loaded plugins namespace
async fn plugin_manager(
    mut plugin_rx: mpsc::UnboundedReceiver<Client>,
    mut user_rx: mpsc::UnboundedReceiver<Command>,
) -> ! {
    let manager = PluginManager::new();

    loop {
        tokio::select! {
            Some(client) = plugin_rx.recv() => {}
            Some(command) = user_rx.recv() => {}
        }
    }
}

/// Steps to connect a client to plugin
/// * Preform http upgrade request and check for `Plugin-Name` header and store to later
/// * Send that stream and header to the plugin manager
async fn handle_connection(stream: TcpStream, tx: mpsc::UnboundedSender<Client>) {
    let mut namespace: String = Default::default();

    let Ok(ws_stream) = accept_hdr_async(stream, |req: &Request, res: Response| {
        let Some(x) = req.headers().get("Plugin-Name") else {
            return Err(ErrorResponse::new(Some("Missing Plugin-Name".to_string())));
        };

        namespace = x
            .to_str()
            .map_err(|e| ErrorResponse::new(Some(e.to_string())))?
            .to_string();

        Ok(res)
    })
    .await
    else {
        return;
    };

    let client = Client {
        socket: ws_stream,
        namespace,
    };

    tx.send(client).unwrap();
}

#[tokio::main]
async fn main() {
    let (ptx, prx) = mpsc::unbounded_channel();
    let (utx, urx) = mpsc::unbounded_channel();

    //let _ = tokio::spawn(plugin_manager(prx, urx));
    thread::spawn(|| user_input(utx));

    let listener = TcpListener::bind(DEFAULT_ADDR)
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on ws://127.0.0.1:8080");

    // Accept connections in a loop
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);
        // Spawn a task for each connection (non-blocking)
        tokio::spawn(handle_connection(stream, ptx.clone()));
    }
}
