use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

// use model::local_types::{ClientEventWrapper, ServerEventWrapper};

pub async fn game_ws(ws: WebSocketUpgrade) -> Response {
    // @TODO: asign role (host/controller) to websoket to control allowed messages
    ws.on_failed_upgrade(|error| eprintln!("Failed upgrade: {}", error))
        .on_upgrade(handle_game_ws)
}

async fn handle_game_ws(mut ws: WebSocket) {
    println!("🚀 New client connected!");

    while let Some(result) = ws.recv().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(payload) => {
                    println!("Received text: {}", payload);
                    if payload == "ping" {
                        let _ = ws.send(Message::Text("pong".into())).await;
                        continue;
                    }
                    // 1. convert payload to client event serde_json::from_string -> Result<ClientEventWrapper>
                    // 2. handle event to get response -> Result<ServerEventWrapper, http::Error>
                    // 3. convert response to playload serde_json::to_string -> Result<String>
                    // 4. send response as Message::Text
                    let _ = ws.send(Message::Text(payload)).await;
                }
                Message::Binary(payload) => {
                    // handle messagePack with rmp-serde
                    println!("Received binary: {}", payload.len());
                    let _ = ws.send(Message::Binary(payload)).await;
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    break;
                }
                Message::Ping(_) => {
                    println!("Received technical Ping");
                }
                Message::Pong(_) => {
                    println!("Received technical Pong");
                }
            },
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
