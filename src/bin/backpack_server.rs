use backpack::parse_stream_name;
use backpack::subscrib_stream::*;
use backpack::UpdataStream;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
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
    let peer_addr = stream.peer_addr()?;
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| anyhow::anyhow!("Error during WebSocket handshake: {}", e))?;
    info!("WebSocket connection established with: {:?}", peer_addr);
    let (tx, rx) = channel::<Message>(1000);
    let (in_tx, in_rx) = broadcast::channel(5);
    let stream_names = Arc::new(Mutex::new(HashMap::<String, Box<dyn UpdataStream>>::new()));
    let stream_names_handle = tokio::spawn(updata_stream(
        stream_names.clone(),
        tx.clone(),
        in_tx.subscribe(),
    ));
    let send_ping_handle = tokio::spawn(send_ping(tx.clone(), in_tx.subscribe()));
    let (write, read) = ws_stream.split();
    let send_message_handle = tokio::spawn(send_message(rx, write, in_rx));
    let read_message_handle = tokio::spawn(read_message(stream_names, read, in_tx));
    let _ = tokio::join!(
        read_message_handle,
        send_message_handle,
        stream_names_handle,
        send_ping_handle
    );
    info!("WebSocket connection closed with: {:?}", peer_addr);
    Ok(())
}

pub async fn subscribe(
    params: Vec<StreamName>,
    stream_names: Arc<Mutex<HashMap<String, Box<dyn UpdataStream>>>>,
) -> anyhow::Result<()> {
    let mut stream_names = stream_names.lock().await;
    info!("Subscribe to stream: {:?}", params);
    for param in params {
        stream_names
            .entry(param.to_string())
            .or_insert_with(|| (parse_stream_name(param.clone())) as _);
    }
    Ok(())
}

pub async fn unsubscribe(
    params: Vec<StreamName>,
    stream_names: Arc<Mutex<HashMap<String, Box<dyn UpdataStream>>>>,
) -> anyhow::Result<()> {
    let mut stream_names = stream_names.lock().await;
    for param in params {
        stream_names.remove(&param.to_string());
    }
    Ok(())
}

pub async fn updata_stream(
    stream_names: Arc<Mutex<HashMap<String, Box<dyn UpdataStream>>>>,
    mut tx: Sender<Message>,
    mut in_rx: broadcast::Receiver<Message>,
) -> anyhow::Result<()> {
    loop {
        let instant = Instant::now();
        sleep(Duration::from_secs(1)).await;
        let mut stream_names = stream_names.lock().await;
        for (_, stream) in stream_names.iter_mut() {
            let rng = rand::thread_rng();
            stream.update(instant.elapsed().as_secs(), rng);
            let message = stream.to_message();
            tx.send(message).await?;
        }
        if in_rx.try_recv().is_ok() {
            break;
        }
    }
    Ok(())
}

pub async fn send_message(
    mut rx: Receiver<Message>,
    mut write: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut in_rx: broadcast::Receiver<Message>,
) {
    while let Some(msg) = rx.next().await {
        write.send(msg).await.unwrap();
        if in_rx.try_recv().is_ok() {
            break;
        }
    }
}

pub async fn send_ping(mut tx: Sender<Message>, mut in_rx: broadcast::Receiver<Message>) {
    loop {
        let msg = Message::Ping("Ping!".as_bytes().to_vec());
        info!("Send a ping message");
        tx.send(msg).await.unwrap();
        sleep(Duration::from_secs(10)).await;
        if in_rx.try_recv().is_ok() {
            break;
        }
    }
}

pub async fn read_message(
    stream_names: Arc<Mutex<HashMap<String, Box<dyn UpdataStream>>>>,
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    in_tx: broadcast::Sender<Message>,
) -> anyhow::Result<()> {
    let mut instant0 = Instant::now();
    while let Some(msg) = read.next().await {
        let now = Instant::now();
        if now.duration_since(instant0).as_secs() > 20 {
            info!("Time out, close the connection");
            in_tx.send(Message::Close(None))?;
            break;
        }
        let names = stream_names.clone();
        let message = msg?;
        match message {
            Message::Text(text) => {
                info!("Received a text message: {}", text);
                let subscrib_stream = serde_json::from_str::<SubscribStream>(&text)?;
                match subscrib_stream.method {
                    Method::Subscribe => subscribe(subscrib_stream.params, names).await?,
                    Method::Unsubscribe => unsubscribe(subscrib_stream.params, names).await?,
                }
            }
            Message::Pong(pong) => {
                if String::from_utf8_lossy(&pong) == "Pong!" {
                    info!(
                        "Received a pong message: {}",
                        String::from_utf8_lossy(&pong)
                    );
                    instant0 += Instant::now() - instant0;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
