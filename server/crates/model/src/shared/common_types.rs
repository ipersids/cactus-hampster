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

// Session creation
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSessionPayload {}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionCreatedPayload {
    pub session_code: String,
}

// Join session
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinSessionPayload {
    pub session_code: String,
    pub player_name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinSuccessPayload {
    pub player_id: String,
    pub session_code: String,
}

// Player info
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInfo {
    pub player_id: String,
    pub player_name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerJoinedPayload {
    pub player: PlayerInfo,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerLeftPayload {
    pub player_id: String,
}

// Player input
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InputType {
    ButtonPress,
    ButtonRelease,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInputPayload {
    pub input_type: InputType,
    pub data: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControllerInputPayload {
    pub player_id: String,
    pub input_type: InputType,
    pub data: String,
}

// Game control
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartGamePayload {}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameStartedPayload {
    pub game_type: String,
}
