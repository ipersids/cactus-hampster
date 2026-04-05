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
    pub message: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PongPayload {
    pub message: String,
}

// Session management payloads

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionCreatedPayload {
    pub session_code: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinSessionPayload {
    pub session_code: String,
    pub nickname: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinSuccessPayload {
    pub player_id: String,
    pub session_code: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerJoinedPayload {
    pub player_id: String,
    pub nickname: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerLeftPayload {
    pub player_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInfo {
    pub player_id: String,
    pub nickname: String,
}

// Game payloads

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartGamePayload {
    pub game_type: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameStartedPayload {
    pub game_type: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInputPayload {
    pub player_id: String,
    pub thrust: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub fire: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControllerInputPayload {
    pub thrust: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub fire: bool,
}
