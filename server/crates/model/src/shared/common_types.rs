use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorPayload {
    pub code: u32,
    pub message: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PingPayload {
    message: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PongPayload {
    message: String,
}
