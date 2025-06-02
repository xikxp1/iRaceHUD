use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex as StdMutex, OnceLock},
};

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use rmp_serde::Serializer;
use serde::Serialize;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type WsClients = Arc<StdMutex<HashMap<SocketAddr, mpsc::UnboundedSender<Vec<u8>>>>>;

static WS_PORT: OnceLock<u16> = OnceLock::new();
pub static WS_SERVER: OnceLock<WebSocketServer> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct WebSocketServer {
    clients: WsClients,
}

impl Default for WebSocketServer {
    fn default() -> Self {
        Self {
            clients: Arc::new(StdMutex::new(HashMap::new())),
        }
    }
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_port() -> Option<u16> {
        WS_PORT.get().copied()
    }

    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind");
        let port = listener.local_addr().unwrap().port();
        WS_PORT.set(port).expect("Failed to set WebSocket port");
        info!(
            "WebSocket server listening on: {}",
            listener.local_addr().unwrap()
        );

        while let Ok((stream, addr)) = listener.accept().await {
            let clients = self.clients.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, addr, clients).await {
                    error!("Error in connection handler: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        clients: WsClients,
    ) -> eyre::Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel();

        {
            clients.lock().unwrap().insert(addr, tx);
            info!("New WebSocket connection: {}", addr);
        }

        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(Message::Binary(msg.into())).await {
                    error!("Error sending message to {}: {}", addr, e);
                    break;
                }
            }
        });

        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(Message::Close(_)) | Err(_) => break,
                _ => continue,
            }
        }

        clients.lock().unwrap().remove(&addr);
        info!("WebSocket connection closed: {}", addr);
        send_task.abort();
        Ok(())
    }

    pub fn broadcast<T: Serialize>(&self, data: &T) {
        let mut buf = Vec::new();
        if data.serialize(&mut Serializer::new(&mut buf)).is_ok() {
            let clients = self.clients.lock().unwrap();
            for (addr, tx) in clients.iter() {
                if let Err(e) = tx.send(buf.clone()) {
                    error!("Failed to send message to {}: {}", addr, e);
                }
            }
        }
    }
}
