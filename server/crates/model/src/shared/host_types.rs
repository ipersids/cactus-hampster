use crate::shared_types::host::{ErrorPayload, PingPayload, PongPayload};
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
}
