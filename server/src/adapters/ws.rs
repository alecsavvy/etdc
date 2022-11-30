use crate::events::Events;
use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::TcpStream;

pub async fn handle_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    read.try_filter(|msg| {
        if let Ok(Ok(msg)) = msg.to_text().map(serde_json::from_str::<Events>) {
            println!("{} sent {:?}", addr, msg);
        }
        future::ready(true)
    })
    .forward(write)
    .await
    .expect("Failed to forward messages")
}
