use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct PingMessage {
    pub msg: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct PongMessage {
    pub msg: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ControllerMessage {
    Ping(PingMessage),
    Pong(PongMessage),
}
