use crate::handler::{controller_ws, host_ws};
use crate::state::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

pub struct GameServerRouter {
    pub service: Router,
}

impl GameServerRouter {
    pub fn init(state: Arc<AppState>) -> Self {
        let service = Router::new()
            .route("/ws/host", get(host_ws))
            .route("/ws/controller", get(controller_ws))
            .with_state(state);
        Self { service }
    }
}
