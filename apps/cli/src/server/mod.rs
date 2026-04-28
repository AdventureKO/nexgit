mod handler;
mod state;
mod stdio;
mod unix;
mod websocket;

use std::{path::PathBuf, sync::Arc};

use anyhow::{bail, Result};

pub(crate) use handler::handle_client_message;
pub(crate) use state::ServerState;

pub async fn run(listen: &str) -> Result<()> {
    let state = Arc::new(ServerState::new());

    match parse_listen_endpoint(listen)? {
        ListenEndpoint::Stdio => stdio::run(state).await,
        ListenEndpoint::WebSocket(addr) => websocket::run(&addr, state).await,
        ListenEndpoint::Unix(path) => unix::run(path, state).await,
        ListenEndpoint::Off => Ok(()),
    }
}

#[derive(Debug)]
enum ListenEndpoint {
    Stdio,
    WebSocket(String),
    Unix(Option<PathBuf>),
    Off,
}

fn parse_listen_endpoint(listen: &str) -> Result<ListenEndpoint> {
    if listen == "stdio://" {
        return Ok(ListenEndpoint::Stdio);
    }

    if listen == "off" {
        return Ok(ListenEndpoint::Off);
    }

    if let Some(addr) = listen.strip_prefix("ws://") {
        if addr.is_empty() {
            bail!("ws:// transport requires an address, for example ws://127.0.0.1:0");
        }
        return Ok(ListenEndpoint::WebSocket(addr.to_string()));
    }

    if let Some(path) = listen.strip_prefix("unix://") {
        let path = if path.is_empty() {
            None
        } else {
            Some(PathBuf::from(path))
        };
        return Ok(ListenEndpoint::Unix(path));
    }

    bail!(
        "unsupported app-server listen endpoint `{}`; expected stdio://, unix://, ws://IP:PORT, or off",
        listen
    )
}
