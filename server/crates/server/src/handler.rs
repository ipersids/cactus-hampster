use crate::state::{AppState, ConnectionId};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use model::shared_types::{
    common::{
        GameStartedPayload, PlayerInputPayload, PlayerJoinedPayload, PlayerLeftPayload,
        SessionCreatedPayload,
    },
    controller::{ControllerEvent, ControllerEventType},
    host::{HostEvent, HostEventType, ServerToHostEvent, ServerToHostEventType},
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn host_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Failed upgrade: {}", error))
        .on_upgrade(move |socket| handle_host_connection(socket, state))
}

pub async fn controller_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Failed upgrade: {}", error))
        .on_upgrade(move |socket| handle_controller_connection(socket, state))
}

async fn handle_host_connection(ws: WebSocket, state: Arc<AppState>) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let conn_id = state.next_conn_id();
    state.register_connection(conn_id, tx).await;

    println!("Host connected: {}", conn_id);

    // Spawn task to forward messages from channel to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Main receive loop
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    if text == "ping" {
                        state.send_to(conn_id, Message::Text("pong".into())).await;
                        continue;
                    }
                    handle_host_message(&state, conn_id, &text).await;
                }
                Message::Close(_) => {
                    println!("Host {} disconnected", conn_id);
                    break;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("WebSocket error for host {}: {}", conn_id, e);
                break;
            }
        }
    }

    send_task.abort();
    state.handle_disconnect(conn_id).await;
}

async fn handle_controller_connection(ws: WebSocket, state: Arc<AppState>) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let conn_id = state.next_conn_id();
    state.register_connection(conn_id, tx).await;

    println!("Controller connected: {}", conn_id);

    // Spawn task to forward messages from channel to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Main receive loop
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    if text == "ping" {
                        state.send_to(conn_id, Message::Text("pong".into())).await;
                        continue;
                    }
                    handle_controller_message(&state, conn_id, &text).await;
                }
                Message::Close(_) => {
                    println!("Controller {} disconnected", conn_id);
                    break;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("WebSocket error for controller {}: {}", conn_id, e);
                break;
            }
        }
    }

    send_task.abort();

    // Handle player disconnect - notify host
    if let Some((session_code, player_id)) = state.handle_disconnect(conn_id).await {
        if let Some(host_conn) = state.get_host_conn(&session_code).await {
            let event = ServerToHostEvent::Success(ServerToHostEventType::PlayerLeft(
                PlayerLeftPayload { player_id },
            ));
            if let Ok(json) = serde_json::to_string(&event) {
                state.send_to(host_conn, Message::Text(json.into())).await;
            }
        }
    }
}

async fn handle_host_message(state: &AppState, conn_id: ConnectionId, text: &str) {
    let event: Result<HostEvent, _> = serde_json::from_str(text);

    match event {
        Ok(HostEvent::Success(event_type)) => match event_type {
            HostEventType::CreateSession => {
                let code = state.create_session(conn_id).await;
                println!("Session created: {}", code);

                let response = ServerToHostEvent::Success(ServerToHostEventType::SessionCreated(
                    SessionCreatedPayload {
                        session_code: code,
                    },
                ));
                if let Ok(json) = serde_json::to_string(&response) {
                    state.send_to(conn_id, Message::Text(json.into())).await;
                }
            }
            HostEventType::StartGame(payload) => {
                if let Some(session_code) = state.get_session_for_conn(conn_id).await {
                    state.start_game(&session_code).await;
                    println!("Game started in session {}: {}", session_code, payload.game_type);

                    // Notify all players
                    let event = model::shared_types::controller::ServerToControllerEvent::Success(
                        model::shared_types::controller::ServerToControllerEventType::GameStarted(
                            GameStartedPayload {
                                game_type: payload.game_type,
                            },
                        ),
                    );
                    if let Ok(json) = serde_json::to_string(&event) {
                        state
                            .broadcast_to_players(&session_code, Message::Text(json.into()))
                            .await;
                    }
                }
            }
            HostEventType::Ping(_) => {
                // Echo pong
            }
            HostEventType::GameStarted(_) => {
                // This is a host->server event, ignore if received from host
            }
        },
        Ok(HostEvent::Error(_)) => {
            // Client sent an error? Ignore
        }
        Err(e) => {
            eprintln!("Failed to parse host message: {} - {}", e, text);
        }
    }
}

async fn handle_controller_message(state: &AppState, conn_id: ConnectionId, text: &str) {
    let event: Result<ControllerEvent, _> = serde_json::from_str(text);

    match event {
        Ok(ControllerEvent::Success(event_type)) => match event_type {
            ControllerEventType::JoinSession(payload) => {
                match state
                    .join_session(&payload.session_code, &payload.nickname, conn_id)
                    .await
                {
                    Ok((player_id, _players)) => {
                        println!(
                            "Player {} ({}) joined session {}",
                            payload.nickname, player_id, payload.session_code
                        );

                        // Send success to controller
                        let response = model::shared_types::controller::ServerToControllerEvent::Success(
                            model::shared_types::controller::ServerToControllerEventType::JoinSuccess(
                                model::shared_types::common::JoinSuccessPayload {
                                    player_id: player_id.clone(),
                                    session_code: payload.session_code.clone(),
                                },
                            ),
                        );
                        if let Ok(json) = serde_json::to_string(&response) {
                            state.send_to(conn_id, Message::Text(json.into())).await;
                        }

                        // Notify host
                        if let Some(host_conn) = state.get_host_conn(&payload.session_code).await {
                            let host_event =
                                ServerToHostEvent::Success(ServerToHostEventType::PlayerJoined(
                                    PlayerJoinedPayload {
                                        player_id,
                                        nickname: payload.nickname,
                                    },
                                ));
                            if let Ok(json) = serde_json::to_string(&host_event) {
                                state.send_to(host_conn, Message::Text(json.into())).await;
                            }
                        }
                    }
                    Err(e) => {
                        let response = model::shared_types::controller::ServerToControllerEvent::Error(
                            model::shared_types::common::ErrorPayload {
                                code: 404,
                                message: e,
                            },
                        );
                        if let Ok(json) = serde_json::to_string(&response) {
                            state.send_to(conn_id, Message::Text(json.into())).await;
                        }
                    }
                }
            }
            ControllerEventType::PlayerInput(input) => {
                // Relay input to host
                if let Some(session_code) = state.get_session_for_conn(conn_id).await {
                    if let Some(player_id) = state.get_player_id_for_conn(conn_id).await {
                        if let Some(host_conn) = state.get_host_conn(&session_code).await {
                            let host_event =
                                ServerToHostEvent::Success(ServerToHostEventType::PlayerInput(
                                    PlayerInputPayload {
                                        player_id,
                                        thrust: input.thrust,
                                        rotate_left: input.rotate_left,
                                        rotate_right: input.rotate_right,
                                        fire: input.fire,
                                    },
                                ));
                            if let Ok(json) = serde_json::to_string(&host_event) {
                                state.send_to(host_conn, Message::Text(json.into())).await;
                            }
                        }
                    }
                }
            }
            ControllerEventType::Ping(_) => {
                // Echo pong
            }
        },
        Ok(ControllerEvent::Error(_)) => {
            // Client sent an error? Ignore
        }
        Err(e) => {
            eprintln!("Failed to parse controller message: {} - {}", e, text);
        }
    }
}
