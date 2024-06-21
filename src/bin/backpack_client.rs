use backpack::subscrib_stream::*;
use clap::Parser;
use futures::channel::mpsc::Sender;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use native_tls::TlsConnector;
use serde_json::Value;
use tokio::fs::OpenOptions;
use tokio::io::{stdin, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::protocol::{Message, WebSocketConfig},
    Connector, MaybeTlsStream, WebSocketStream,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opt = Opts::parse();
    let url = opt.url;
    info!("User input url: {}", url);
    let stream_name = StreamName::from(opt.stream_name);
    let method = Method::from(opt.method);

    let (ws_stream, _) = connect_async_tls_with_config(
        url,
        Some(WebSocketConfig::default()),
        false,
        Some(Connector::NativeTls(TlsConnector::new().unwrap())),
    )
    .await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();

    let subscrib_stream = SubscribStream {
        method,
        params: vec![stream_name],
    };
    let json = serde_json::to_string(&subscrib_stream).unwrap();
    let msg = Message::Text(json);
    ws_write.send(msg).await?;
    info!("Subscribed!");
    let mut sum = 0usize;
    let instant = Instant::now();
    while let Some(msg) = ws_read.next().await {
        let message = msg?;
        match message {
            Message::Text(text) => {
                sum += 1;
                if sum % 20 == 0 {
                    info!("Received 10 messages since last time");
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
                if instant.elapsed().as_secs() < 60 {
                    info!("Received a ping message:{}", String::from_utf8_lossy(&ping));
                    let msg = Message::Pong("Pong!".as_bytes().to_vec());
                    ws_write.send(msg).await?;
                    info!("Sent a pong message");
                } else {
                    info!("Time out, close the connection");
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn read_message(
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    mut ws_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) -> anyhow::Result<()> {
    let mut sum = 0usize;
    while let Some(msg) = ws_read.next().await {
        let message = msg?;
        match message {
            Message::Text(text) => {
                sum += 1;
                if sum % 10 == 0 {
                    info!("Received 10 messages since last time");
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

pub async fn read_stdin(mut sender: Sender<String>) -> anyhow::Result<()> {
    let mut stdin = stdin();

    loop {
        let mut buf = vec![0; 1024];

        let n = stdin.read(&mut buf).await?;
        buf.truncate(n);
        let msg = String::from_utf8(buf).unwrap();
        sender.send(msg).await?;
    }
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short, long, default_value = "wss://ws.backpack.exchange")]
    url: String,
    #[clap(short, long, default_value = "depth.SOL_USDC")]
    stream_name: String,
    #[clap(short, long, default_value = "SUBSCRIBE")]
    method: String,
}
