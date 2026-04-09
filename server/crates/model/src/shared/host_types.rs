use crate::shared_types::common::{
    ControllerInputPayload, ErrorPayload, PingPayload, PlayerJoinedPayload, PlayerLeftPayload,
    PongPayload, CreateSessionPayload, SessionCreatedPayload, StartGamePayload,
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
    ControllerInput(ControllerInputPayload),
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
    CreateSession(CreateSessionPayload),
    StartGame(StartGamePayload),
}
