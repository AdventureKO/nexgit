use std::{path::PathBuf, sync::Arc};

use anyhow::Result;

use super::ServerState;

#[cfg(unix)]
pub async fn run(path: Option<PathBuf>, state: Arc<ServerState>) -> Result<()> {
    use nexgit_protocol::ServerMessage;
    use tokio::net::UnixListener;

    let path = path.unwrap_or_else(|| {
        std::env::temp_dir().join(format!("nexgit-{}.sock", std::process::id()))
    });

    if path.exists() {
        std::fs::remove_file(&path)?;
    }

    let listener = UnixListener::bind(&path)?;
    let endpoint = format!("unix://{}", path.display());

    println!(
        "{}",
        serde_json::to_string(&ServerMessage::ready("unix", Some(endpoint.clone())))?
    );
    tracing::info!(%endpoint, "nexgit app-server listening");

    loop {
        let (stream, _) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let (reader, writer) = stream.into_split();
            if let Err(error) =
                super::stdio::run_line_protocol(reader, writer, state, "unix", None).await
            {
                tracing::warn!(%error, "unix socket client failed");
            }
        });
    }
}

#[cfg(not(unix))]
pub async fn run(_path: Option<PathBuf>, _state: Arc<ServerState>) -> Result<()> {
    anyhow::bail!("unix:// app-server transport is only supported on Unix-like systems")
}
