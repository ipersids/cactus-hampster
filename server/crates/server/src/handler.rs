use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::state::{AppState, ControllerMessage, HostMessage};
use model::shared_types::{
    common::{
        ControllerInputPayload, ErrorPayload, GameStartedPayload, JoinSuccessPayload, PlayerInfo,
        PlayerJoinedPayload, PlayerLeftPayload, PongPayload, SessionCreatedPayload,
    },
    controller::{
        ControllerEvent, ControllerEventType, ServerToControllerEvent, ServerToControllerEventType,
    },
    host::{HostEvent, HostEventType, ServerToHostEvent, ServerToHostEventType},
};

// ===== HOST HANDLER =====

pub async fn host_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Host upgrade failed: {}", error))
        .on_upgrade(move |socket| handle_host_ws(socket, state))
}

async fn handle_host_ws(ws: WebSocket, state: Arc<AppState>) {
    println!("Host connected!");

    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<HostMessage>();

    let mut session_code: Option<String> = None;

    // Task to forward messages from channel to WebSocket
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

    // Process incoming messages
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(payload)) => {
                let payload_str: &str = &payload;
                if payload_str == "ping" {
                    let _ = tx.send(HostMessage::Event("pong".to_string()));
                    continue;
                }

                match serde_json::from_str::<HostEvent>(payload_str) {
                    Ok(HostEvent::Success(event_type)) => match event_type {
                        HostEventType::Ping(ping) => {
                            let response = ServerToHostEvent::Success(ServerToHostEventType::Pong(
                                PongPayload {
                                    message: ping.message,
                                },
                            ));
                            let _ = tx.send(HostMessage::Event(
                                serde_json::to_string(&response).unwrap(),
                            ));
                        }
                        HostEventType::CreateSession(_) => {
                            let code = state.generate_session_code().await;
                            state.create_session(code.clone(), tx.clone()).await;
                            session_code = Some(code.clone());

                            let response = ServerToHostEvent::Success(
                                ServerToHostEventType::SessionCreated(SessionCreatedPayload {
                                    session_code: code,
                                }),
                            );
                            let _ = tx.send(HostMessage::Event(
                                serde_json::to_string(&response).unwrap(),
                            ));
                        }
                        HostEventType::StartGame(_) => {
                            if let Some(ref code) = session_code {
                                let response = ServerToControllerEvent::Success(
                                    ServerToControllerEventType::GameStarted(GameStartedPayload {
                                        game_type: "default".to_string(),
                                    }),
                                );
                                state
                                    .broadcast_to_controllers(
                                        code,
                                        &serde_json::to_string(&response).unwrap(),
                                    )
                                    .await;
                            }
                        }
                    },
                    Ok(HostEvent::Error(_)) => {
                        // Client sent an error - log it
                    }
                    Err(e) => {
                        eprintln!("Failed to parse host event: {}", e);
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("Host WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Cleanup
    if let Some(code) = session_code {
        state.remove_session(&code).await;
    }
    send_task.abort();
    println!("Host disconnected");
}

// ===== CONTROLLER HANDLER =====

pub async fn controller_ws(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_failed_upgrade(|error| eprintln!("Controller upgrade failed: {}", error))
        .on_upgrade(move |socket| handle_controller_ws(socket, state))
}

async fn handle_controller_ws(ws: WebSocket, state: Arc<AppState>) {
    println!("Controller connected!");

    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ControllerMessage>();

    let mut session_code: Option<String> = None;
    let mut player_id: Option<String> = None;

    // Task to forward messages from channel to WebSocket
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

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(payload)) => {
                let payload_str: &str = &payload;
                if payload_str == "ping" {
                    let _ = tx.send(ControllerMessage::Event("pong".to_string()));
                    continue;
                }

                match serde_json::from_str::<ControllerEvent>(payload_str) {
                    Ok(ControllerEvent::Success(event_type)) => match event_type {
                        ControllerEventType::Ping(ping) => {
                            let response = ServerToControllerEvent::Success(
                                ServerToControllerEventType::Pong(PongPayload {
                                    message: ping.message,
                                }),
                            );
                            let _ = tx.send(ControllerMessage::Event(
                                serde_json::to_string(&response).unwrap(),
                            ));
                        }
                        ControllerEventType::JoinSession(join) => {
                            match state
                                .add_player_to_session(
                                    &join.session_code,
                                    join.player_name.clone(),
                                    tx.clone(),
                                )
                                .await
                            {
                                Ok(player) => {
                                    session_code = Some(join.session_code.clone());
                                    player_id = Some(player.id.clone());

                                    // Send success to controller
                                    let response = ServerToControllerEvent::Success(
                                        ServerToControllerEventType::JoinSuccess(
                                            JoinSuccessPayload {
                                                player_id: player.id.clone(),
                                                session_code: join.session_code.clone(),
                                            },
                                        ),
                                    );
                                    let _ = tx.send(ControllerMessage::Event(
                                        serde_json::to_string(&response).unwrap(),
                                    ));

                                    // Notify host
                                    if let Some(host_tx) =
                                        state.get_host_sender(&join.session_code).await
                                    {
                                        let host_event = ServerToHostEvent::Success(
                                            ServerToHostEventType::PlayerJoined(
                                                PlayerJoinedPayload {
                                                    player: PlayerInfo {
                                                        player_id: player.id,
                                                        player_name: join.player_name,
                                                    },
                                                },
                                            ),
                                        );
                                        let _ = host_tx.send(HostMessage::Event(
                                            serde_json::to_string(&host_event).unwrap(),
                                        ));
                                    }
                                }
                                Err(e) => {
                                    let response = ServerToControllerEvent::Error(ErrorPayload {
                                        code: 404,
                                        message: e,
                                    });
                                    let _ = tx.send(ControllerMessage::Event(
                                        serde_json::to_string(&response).unwrap(),
                                    ));
                                }
                            }
                        }
                        ControllerEventType::PlayerInput(input) => {
                            if let (Some(code), Some(pid)) = (&session_code, &player_id) {
                                if let Some(host_tx) = state.get_host_sender(code).await {
                                    let host_event = ServerToHostEvent::Success(
                                        ServerToHostEventType::ControllerInput(
                                            ControllerInputPayload {
                                                player_id: pid.clone(),
                                                input_type: input.input_type,
                                                data: input.data,
                                            },
                                        ),
                                    );
                                    let _ = host_tx.send(HostMessage::Event(
                                        serde_json::to_string(&host_event).unwrap(),
                                    ));
                                }
                            }
                        }
                    },
                    Ok(ControllerEvent::Error(_)) => {}
                    Err(e) => {
                        eprintln!("Failed to parse controller event: {}", e);
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("Controller WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Cleanup - notify host about player leaving
    if let (Some(code), Some(pid)) = (session_code, player_id) {
        if state.remove_player_from_session(&code, &pid).await.is_some() {
            if let Some(host_tx) = state.get_host_sender(&code).await {
                let host_event = ServerToHostEvent::Success(ServerToHostEventType::PlayerLeft(
                    PlayerLeftPayload { player_id: pid },
                ));
                let _ = host_tx.send(HostMessage::Event(
                    serde_json::to_string(&host_event).unwrap(),
                ));
            }
        }
    }
    send_task.abort();
    println!("Controller disconnected");
}
