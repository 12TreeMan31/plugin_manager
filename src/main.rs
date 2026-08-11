/*
    Client Connects and opens websocket
    Server waits for client to provide the plugin they would like to use
    Server then checks if plugin is valid. Closes connection if can't, spawns task with plugin if can
    Task ends when the client closes the stream
*/

use plugin_manager::client::*;
use plugin_manager::commands::*;
use plugin_manager::plugin::*;
use std::thread;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

const DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    // Channels used to talk with `plugin::plugin_handler`
    let (plg_tx, plg_rx) = mpsc::unbounded_channel();

    // Needs to be sync, see `tokio::io::stdin`
    let cl_plg_tx = plg_tx.clone();
    thread::spawn(move || user_input(cl_plg_tx));

    tokio::spawn(plugin_handler(plg_rx));

    let listener = TcpListener::bind(DEFAULT_ADDR)
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on ws://127.0.0.1:8080");

    // Accept connections in a loop
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);
        // Spawn a task for each connection (non-blocking)
        tokio::spawn(handle_connection(stream, plg_tx.clone()));
    }
}
