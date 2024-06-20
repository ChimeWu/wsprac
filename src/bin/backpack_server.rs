use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable logging
    tracing_subscriber::fmt::init();
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        info!("Accepted connection from: {}", stream.peer_addr()?);
        tokio::spawn(process(stream));
    }

    Ok(())
}

async fn process(stream: TcpStream) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .map_err(|e| anyhow::anyhow!("Error during WebSocket handshake: {}", e))?;
    info!("WebSocket connection established");

    let (write, read) = ws_stream.split();
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages");

    Ok(())
}
