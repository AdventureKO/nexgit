use std::sync::Arc;

use anyhow::Result;
use nexgit_protocol::{ClientMessage, ServerMessage};
use tokio::io::{
    self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter,
};

use super::{handle_client_message, ServerState};

pub async fn run(state: Arc<ServerState>) -> Result<()> {
    run_line_protocol(io::stdin(), io::stdout(), state, "stdio", None).await
}

pub(crate) async fn run_line_protocol<R, W>(
    reader: R,
    writer: W,
    state: Arc<ServerState>,
    transport: &str,
    endpoint: Option<String>,
) -> Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut lines = BufReader::new(reader).lines();
    let mut writer = BufWriter::new(writer);

    write_message(&mut writer, &ServerMessage::ready(transport, endpoint)).await?;

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<ClientMessage>(line) {
            Ok(message) => handle_client_message(state.as_ref(), message).await,
            Err(error) => ServerMessage::failure(0, "invalid_json", error.to_string()),
        };

        write_message(&mut writer, &response).await?;
    }

    Ok(())
}

async fn write_message<W>(writer: &mut BufWriter<W>, message: &ServerMessage) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let json = serde_json::to_string(message)?;
    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}
