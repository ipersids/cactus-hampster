use crate::handler::pong;
use axum::{Router, routing::get};

pub struct GameServerRouter {
    pub service: Router,
}

impl GameServerRouter {
    pub fn init() -> Self {
        let service = Router::new().route("/ping", get(pong));
        Self { service }
    }
}
