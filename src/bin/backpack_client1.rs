use futures::{SinkExt, StreamExt};
use native_tls::TlsConnector;
use serde_json::Value;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::protocol::{Message, WebSocketConfig},
    Connector,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "wss://ws.backpack.exchange".to_string());

    let (ws_stream, _) = connect_async_tls_with_config(
        url,
        Some(WebSocketConfig::default()),
        false,
        Some(Connector::NativeTls(TlsConnector::new().unwrap())),
    )
    .await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();

    let subscribe_msg = r#"{"method":"SUBSCRIBE","params":["depth.SOL_USDC"]}"#.to_string();
    let msg = Message::Text(subscribe_msg);
    ws_write.send(msg).await?;
    info!("Subscribed to the depth.SOL_USDC channel");
    let mut sum = 0usize;
    while let Some(msg) = ws_read.next().await {
        let message = msg?;
        match message {
            Message::Text(text) => {
                sum += 1;
                if sum % 1000 == 0 {
                    info!("Received 1000 messages since last time");
                }
                let v = serde_json::from_str::<Value>(&text)?;
                let mut pretty_msg = serde_json::to_string_pretty(&v)?;
                pretty_msg += "\n";
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open("./message.json")
                    .await?;
                file.write_all(pretty_msg.as_bytes()).await?;
            }
            Message::Ping(ping) => {
                info!("Received a ping message:{}", String::from_utf8_lossy(&ping));
                let msg = Message::Pong("Pong!".as_bytes().to_vec());
                ws_write.send(msg).await?;
                info!("Sent a pong message");
            }
            _ => {}
        }
    }
    Ok(())
}
