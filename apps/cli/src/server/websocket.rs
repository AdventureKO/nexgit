use std::sync::Arc;

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use nexgit_protocol::{ClientMessage, ServerMessage};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use super::{handle_client_message, ServerState};

pub async fn run(addr: &str, state: Arc<ServerState>) -> Result<()> {
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind websocket app-server to {addr}"))?;
    let local_addr = listener.local_addr()?;
    let endpoint = format!("ws://{local_addr}");

    println!(
        "{}",
        serde_json::to_string(&ServerMessage::ready("websocket", Some(endpoint.clone())))?
    );
    tracing::info!(%endpoint, "nexgit app-server listening");

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(error) = handle_connection(stream, state).await {
                tracing::warn!(%peer_addr, %error, "websocket client failed");
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, state: Arc<ServerState>) -> Result<()> {
    let mut websocket = accept_async(stream).await?;

    websocket
        .send(Message::Text(
            serde_json::to_string(&ServerMessage::ready("websocket", None))?.into(),
        ))
        .await?;

    while let Some(message) = websocket.next().await {
        match message? {
            Message::Text(text) => {
                let response = handle_text_message(text.as_ref(), state.as_ref()).await;
                websocket
                    .send(Message::Text(serde_json::to_string(&response)?.into()))
                    .await?;
            }
            Message::Binary(bytes) => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    let response = handle_text_message(&text, state.as_ref()).await;
                    websocket
                        .send(Message::Text(serde_json::to_string(&response)?.into()))
                        .await?;
                }
            }
            Message::Ping(payload) => websocket.send(Message::Pong(payload)).await?,
            Message::Close(_) => break,
            Message::Pong(_) | Message::Frame(_) => {}
        }
    }

    Ok(())
}

async fn handle_text_message(text: &str, state: &ServerState) -> ServerMessage {
    match serde_json::from_str::<ClientMessage>(text) {
        Ok(message) => handle_client_message(state, message).await,
        Err(error) => ServerMessage::failure(0, "invalid_json", error.to_string()),
    }
}
