use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

use crate::state::{AppState, HostMessage};
use model::shared_types::{
    common::{GameStartedPayload, PingPayload, PongPayload, SessionCreatedPayload},
    controller::ServerToControllerEventType,
    host::{HostEvent, HostEventType, ServerToHostEventType},
};

use super::{IntoControllerResponse, IntoHostResponse};

pub async fn host_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Host upgrade failed: {}", error))
        .on_upgrade(move |socket| run_host_connection(socket, state))
}

async fn run_host_connection(ws: WebSocket, state: Arc<AppState>) {
    println!("Host connected!");

    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<HostMessage>();

    let mut session_code: Option<String> = None;

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                HostMessage::Event(text) => {
                    if ws_sender.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
                HostMessage::Close => break,
            }
        }
    });

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(payload) => {
                let payload_str: &str = &payload;

                if payload_str == "ping" {
                    let _ = tx.send(HostMessage::Event("pong".to_string()));
                    continue;
                }

                if let Ok(event) = serde_json::from_str::<HostEvent>(payload_str) {
                    handle_host_event(event, &state, &tx, &mut session_code).await;
                } else {
                    eprintln!("Failed to parse host event");
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    if let Some(code) = session_code {
        state.remove_session(&code).await;
    }
    send_task.abort();
    println!("Host disconnected");
}

async fn handle_host_event(
    event: HostEvent,
    state: &Arc<AppState>,
    tx: &UnboundedSender<HostMessage>,
    session_code: &mut Option<String>,
) {
    let HostEvent::Success(event_type) = event else {
        return;
    };

    match event_type {
        HostEventType::Ping(ping) => handle_ping(tx, ping),
        HostEventType::CreateSession(_) => handle_create_session(state, tx, session_code).await,
        HostEventType::StartGame(_) => handle_start_game(state, session_code).await,
    }
}

fn handle_ping(tx: &UnboundedSender<HostMessage>, ping: PingPayload) {
    let response = ServerToHostEventType::Pong(PongPayload {
        message: ping.message,
    });
    let _ = tx.send(HostMessage::Event(response.into_response()));
}

async fn handle_create_session(
    state: &Arc<AppState>,
    tx: &UnboundedSender<HostMessage>,
    session_code: &mut Option<String>,
) {
    let code = state.generate_session_code().await;
    state.create_session(code.clone(), tx.clone()).await;
    *session_code = Some(code.clone());

    let response = ServerToHostEventType::SessionCreated(SessionCreatedPayload {
        session_code: code,
    });
    let _ = tx.send(HostMessage::Event(response.into_response()));
}

async fn handle_start_game(state: &Arc<AppState>, session_code: &Option<String>) {
    let Some(code) = session_code else { return };

    let response = ServerToControllerEventType::GameStarted(GameStartedPayload {
        game_type: "default".to_string(),
    });
    state
        .broadcast_to_controllers(code, &response.into_response())
        .await;
}
