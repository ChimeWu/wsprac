use futures_util::{Sink, SinkExt, Stream, StreamExt};
use tokio::io::{stdin, stdout, AsyncReadExt, AsyncWriteExt};
use tokio::join;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Please provide a URL"));
    let (tx, rx) = channel(32);
    let read_stdin = tokio::spawn(read_stdin(tx));
    let (ws_stream, _) = connect_async(url).await?;
    let (write, read) = ws_stream.split();
    let ws_to_stdout_handle = tokio::spawn(ws_to_stdout(read));
    let stdin_to_ws_handle = tokio::spawn(stdin_to_ws(rx, write));
    let _ = join!(ws_to_stdout_handle, stdin_to_ws_handle, read_stdin);
    Ok(())
}

async fn read_stdin(sender: Sender<Message>) -> anyhow::Result<()> {
    let mut stdin = stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = stdin.read(&mut buf).await?;
        info!("Read {} bytes from stdin", n);
        buf.truncate(n);
        sender.send(Message::binary(buf)).await?;
    }
}

async fn stdin_to_ws(
    rx: Receiver<Message>,
    mut write: impl Sink<Message> + Unpin,
) -> anyhow::Result<()> {
    let mut stream = ReceiverStream::from(rx);
    while let Some(msg) = stream.next().await {
        if (write.send(msg).await).is_err() {
            eprintln!("Error!");
            break;
        }
    }
    Ok(())
}

async fn ws_to_stdout(
    mut read: impl Stream<Item = Result<Message, Error>> + Unpin,
) -> anyhow::Result<()> {
    while let Some(msg) = read.next().await {
        let data = msg.unwrap().into_data();
        stdout().write_all(&data).await?;
    }
    Ok(())
}
