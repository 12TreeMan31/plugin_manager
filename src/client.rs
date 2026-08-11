use crate::plugin::PluginMessage;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{WebSocketStream, accept_hdr_async};
use tungstenite::Message;
use tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tungstenite::http::HeaderMap;

fn status_fmt(kind: &str, msg: &str) -> String {
    format!("{{\"kind\":\"{kind}\",\"msg\":\"{msg}\"}}")
}

/// One thing you might noticed is that there are many spots when creating the connection
/// where we close the socket if there is an error instead of setting it to Client<Disconnected>.
/// The reasoning for this is that we want to keep a socket alive for as long as possble, and try
/// to handle the error without closing the socket
enum Disconnected {
    MissingPlugin,
    Error(String),
}

struct Connected;
struct Registered;
struct Client<State> {
    stream: WebSocketStream<TcpStream>,
    computer: String,
    plugin: String,

    state: State,
}

impl Client<Connected> {
    fn new(stream: WebSocketStream<TcpStream>, computer: String, namespace: String) -> Self {
        Self {
            stream,
            computer,
            plugin: namespace,
            state: Connected,
        }
    }

    pub async fn register(
        mut self,
        plg_tx: &mpsc::UnboundedSender<(PluginMessage, oneshot::Sender<Option<String>>)>,
    ) -> Option<Client<Registered>> {
        // Sets up the message
        println!("1");
        let (tx, rx) = oneshot::channel();
        let msg = PluginMessage::Exists {
            plugin: self.plugin.clone(),
        };

        println!("2");
        if let Err(e) = plg_tx.send((msg, tx)) {
            println!("Error: {}", e);
            return None;
        }

        println!("3");
        let Some(ret) = rx.await.expect("Wont fail") else {
            println!("Did not get a string");
            return None;
        };

        println!("4");
        println!("{}", ret);
        if ret == "false" {
            let err = Message::Text(r#"{"kind":"error","msg":"plugin is not registered"}"#.into());
            let _ = self.stream.send(err).await;
            tokio::time::sleep(tokio::time::Duration::new(1, 0)).await;
            let _ = self.stream.close(None).await;
            return None;
        }

        println!("5");
        let ok = Message::Text(r#"{"kind":"ok","msg":"success"}"#.into());

        println!("6");
        let _ = self.stream.send(ok).await;

        Some(Client {
            state: Registered,
            ..self
        })
    }
}

impl Client<Registered> {
    async fn handle_request(
        &mut self,
        plg_tx: mpsc::UnboundedSender<(PluginMessage, oneshot::Sender<Option<String>>)>,
    ) {
        let Some(Ok(res)) = self.stream.next().await else {
            return;
        };

        if !res.is_text() {
            return;
        }
        let msg = res.into_text().unwrap().to_string();
        let (func, json) = msg.split_once('-').unwrap();

        let plg_msg = PluginMessage::Message {
            plugin: self.plugin.clone(),
            func: func.to_string(),
            data: json.to_string(),
        };
        let (tx, rx) = oneshot::channel();
        plg_tx.send((plg_msg, tx)).unwrap();

        let ret = rx.await.unwrap().unwrap();
        self.stream.send(Message::Text(ret.into())).await;
    }
}

fn extract_headers(headers: &HeaderMap) -> Result<(String, String), String> {
    // See `HeaderName::as_str()` for why this is lowercase
    const PLG_HEADER: &str = "plugin-name";
    const COM_HEADER: &str = "computer-name";

    let mut plugin = None;
    let mut computer = None;

    for (key, val) in headers {
        match key.as_str() {
            PLG_HEADER => {
                plugin = Some(val.to_str().map_err(|e| e.to_string())?.to_string());
            }
            COM_HEADER => {
                computer = Some(val.to_str().map_err(|e| e.to_string())?.to_string());
            }
            _ => (),
        }
    }

    match (plugin, computer) {
        (Some(p), Some(c)) => Ok((p, c)),
        _ => Err("Missing Plugin-Name or Computer-Name header".to_string()),
    }
}

/// Steps to connect a client to a plugin
/// * Preform http upgrade request and check for `Plugin-Name` and 'Computer-Name'
/// * Send that stream and header to the plugin manager
async fn accept_connection(stream: TcpStream) -> Result<Client<Connected>, String> {
    let mut plugin = Default::default();
    let mut computer = Default::default();

    let ws_stream = accept_hdr_async(stream, |req: &Request, res: Response| {
        let headers = req.headers();
        (plugin, computer) = extract_headers(headers).map_err(|e| ErrorResponse::new(Some(e)))?;

        Ok(res)
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(Client::new(ws_stream, computer, plugin))
}

pub async fn handle_connection(
    stream: TcpStream,
    plg_tx: mpsc::UnboundedSender<(PluginMessage, oneshot::Sender<Option<String>>)>,
) {
    let client = match accept_connection(stream).await {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let Some(mut client) = client.register(&plg_tx).await else {
        println!("Unable to register client");
        return;
    };

    client
        .stream
        .send(Message::Text(status_fmt("ok", "yay").into()))
        .await;

    loop {
        client.handle_request(plg_tx.clone()).await;
    }
}
