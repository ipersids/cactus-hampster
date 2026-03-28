use crate::shared_types::host::{ErrorPayload, PingPayload, PongPayload};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum ServerToControllerEvent {
    #[serde(rename = "success")]
    Success(ServerToControllerEventType),
    #[serde(rename = "error")]
    Error(ErrorPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ServerToControllerEventType {
    Pong(PongPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "data", rename_all = "camelCase")]
pub enum ControllerEvent {
    #[serde(rename = "success")]
    Success(ControllerEventType),
    #[serde(rename = "error")]
    Error(ErrorPayload),
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ControllerEventType {
    Ping(PingPayload),
}
