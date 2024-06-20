use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{stdin, stdout, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Please provide a URL"));
    info!("User input url: {}", url);

    let (tx, rx) = channel(32);
    tokio::spawn(read_stdin(tx));
    info!("Spawned read_stdin task");
    info!("Is ready for WebSocket connecting");
    let (ws_stream, _) = connect_async(url).await?;
    info!("Connected to WebSocket");
    let (write, read) = ws_stream.split();

    let ws_to_stdout = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        info!("Received a message from WebSocket");
        stdout().write_all(&data).await.unwrap();
        info!("Wrote message to stdout");
    });

    let stdin_to_ws = ReceiverStream::from(rx).map(Ok).forward(write);

    pin_mut!(ws_to_stdout, stdin_to_ws);
    future::select(ws_to_stdout, stdin_to_ws).await;

    Ok(())
}

async fn read_stdin(sender: Sender<Message>) -> anyhow::Result<()> {
    let mut stdin = stdin();
    info!("Reading is Ready");
    let mut account = 0;
    loop {
        account += 1;
        info!("Reading from stdin: {}", account);
        let mut buf = vec![0; 1024];
        let n = stdin.read(&mut buf).await?;
        info!("Read {} bytes from stdin", n);
        buf.truncate(n);
        sender.send(Message::binary(buf)).await?;
        info!("Sent message to Channel")
    }
}
