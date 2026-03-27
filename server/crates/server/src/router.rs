use crate::handler::game_ws;
use axum::{Router, routing::get};

pub struct GameServerRouter {
    pub service: Router,
}

impl GameServerRouter {
    pub fn init() -> Self {
        // @TODO: set payload size limit
        let service = Router::new().route("/ping", get(game_ws));
        Self { service }
    }
}
