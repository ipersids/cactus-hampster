use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn pong(ws: WebSocketUpgrade) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Failed upgrade: {}", error))
        .on_upgrade(handle_pong)
}

async fn handle_pong(mut ws: WebSocket) {
    println!("🚀 New client connected!");

    while let Some(result) = ws.recv().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(t) => {
                    println!("Received text: {}", t);
                    let _ = ws.send(Message::Text("pong".into())).await;
                }
                Message::Binary(b) => {
                    println!("Received binary: {}", b.len());
                }
                Message::Ping(_) => {
                    println!("Received technical Ping");
                }
                Message::Pong(_) => {
                    println!("Received technical Pong");
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    break;
                }
            },
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
