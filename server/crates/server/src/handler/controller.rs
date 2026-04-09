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

use crate::state::{AppState, ControllerMessage, HostMessage};
use model::shared_types::{
    common::{
        ControllerInputPayload, ErrorPayload, JoinSessionPayload, JoinSuccessPayload,
        PingPayload, PlayerInfo, PlayerInputPayload, PlayerJoinedPayload, PlayerLeftPayload,
        PongPayload,
    },
    controller::{ControllerEvent, ControllerEventType, ServerToControllerEventType},
    host::ServerToHostEventType,
};

use super::{IntoControllerResponse, IntoHostResponse};

struct ControllerSession {
    session_code: Option<String>,
    player_id: Option<String>,
}

pub async fn controller_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Controller upgrade failed: {}", error))
        .on_upgrade(move |socket| run_controller_connection(socket, state))
}

async fn run_controller_connection(ws: WebSocket, state: Arc<AppState>) {
    println!("Controller connected!");

    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ControllerMessage>();

    let mut session = ControllerSession {
        session_code: None,
        player_id: None,
    };

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                ControllerMessage::Event(text) => {
                    if ws_sender.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
                ControllerMessage::Close => break,
            }
        }
    });

    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(payload) => {
                let payload_str: &str = &payload;

                if payload_str == "ping" {
                    let _ = tx.send(ControllerMessage::Event("pong".to_string()));
                    continue;
                }

                if let Ok(event) = serde_json::from_str::<ControllerEvent>(payload_str) {
                    handle_controller_event(event, &state, &tx, &mut session).await;
                } else {
                    eprintln!("Failed to parse controller event");
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    handle_disconnect(&state, session).await;
    send_task.abort();
    println!("Controller disconnected");
}

async fn handle_controller_event(
    event: ControllerEvent,
    state: &Arc<AppState>,
    tx: &UnboundedSender<ControllerMessage>,
    session: &mut ControllerSession,
) {
    let ControllerEvent::Success(event_type) = event else {
        return;
    };

    match event_type {
        ControllerEventType::Ping(ping) => handle_ping(tx, ping),
        ControllerEventType::JoinSession(join) => handle_join_session(state, tx, session, join).await,
        ControllerEventType::PlayerInput(input) => handle_player_input(state, session, input).await,
    }
}

fn handle_ping(tx: &UnboundedSender<ControllerMessage>, ping: PingPayload) {
    let response = ServerToControllerEventType::Pong(PongPayload {
        message: ping.message,
    });
    let _ = tx.send(ControllerMessage::Event(response.into_response()));
}

async fn handle_join_session(
    state: &Arc<AppState>,
    tx: &UnboundedSender<ControllerMessage>,
    session: &mut ControllerSession,
    join: JoinSessionPayload,
) {
    let result = state
        .add_player_to_session(&join.session_code, join.player_name.clone(), tx.clone())
        .await;

    match result {
        Ok(player) => {
            session.session_code = Some(join.session_code.clone());
            session.player_id = Some(player.id.clone());

            // Respond to controller
            let response = ServerToControllerEventType::JoinSuccess(JoinSuccessPayload {
                player_id: player.id.clone(),
                session_code: join.session_code.clone(),
            });
            let _ = tx.send(ControllerMessage::Event(response.into_response()));

            // Notify host
            notify_host_player_joined(state, &join.session_code, player.id, join.player_name).await;
        }
        Err(e) => {
            let error = ErrorPayload { code: 404, message: e };
            let _ = tx.send(ControllerMessage::Event(error.into_response()));
        }
    }
}

async fn notify_host_player_joined(
    state: &Arc<AppState>,
    session_code: &str,
    player_id: String,
    player_name: String,
) {
    let Some(host_tx) = state.get_host_sender(session_code).await else {
        return;
    };

    let event = ServerToHostEventType::PlayerJoined(PlayerJoinedPayload {
        player: PlayerInfo {
            player_id,
            player_name,
        },
    });
    let _ = host_tx.send(HostMessage::Event(event.into_response()));
}

async fn handle_player_input(
    state: &Arc<AppState>,
    session: &ControllerSession,
    input: PlayerInputPayload,
) {
    let (Some(code), Some(pid)) = (&session.session_code, &session.player_id) else {
        return;
    };

    let Some(host_tx) = state.get_host_sender(code).await else {
        return;
    };

    let event = ServerToHostEventType::ControllerInput(ControllerInputPayload {
        player_id: pid.clone(),
        input_type: input.input_type,
        data: input.data,
    });
    let _ = host_tx.send(HostMessage::Event(event.into_response()));
}

async fn handle_disconnect(state: &Arc<AppState>, session: ControllerSession) {
    let (Some(code), Some(pid)) = (session.session_code, session.player_id) else {
        return;
    };

    if state.remove_player_from_session(&code, &pid).await.is_none() {
        return;
    }

    let Some(host_tx) = state.get_host_sender(&code).await else {
        return;
    };

    let event = ServerToHostEventType::PlayerLeft(PlayerLeftPayload { player_id: pid });
    let _ = host_tx.send(HostMessage::Event(event.into_response()));
}
