use crate::shared_types::common::{
    ErrorPayload, GameStartedPayload, PingPayload, PlayerInputPayload, PlayerJoinedPayload,
    PlayerLeftPayload, PongPayload, SessionCreatedPayload, StartGamePayload,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum ServerToHostEvent {
    #[serde(rename = "success")]
    Success(ServerToHostEventType),
    #[serde(rename = "error")]
    Error(ErrorPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerToHostEventType {
    Pong(PongPayload),
    SessionCreated(SessionCreatedPayload),
    PlayerJoined(PlayerJoinedPayload),
    PlayerLeft(PlayerLeftPayload),
    PlayerInput(PlayerInputPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum HostEvent {
    #[serde(rename = "success")]
    Success(HostEventType),
    #[serde(rename = "error")]
    Error(ErrorPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum HostEventType {
    Ping(PingPayload),
    CreateSession,
    StartGame(StartGamePayload),
    GameStarted(GameStartedPayload),
}
