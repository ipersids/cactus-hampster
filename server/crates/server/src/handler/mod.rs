mod controller;
mod host;

pub use controller::controller_ws;
pub use host::host_ws;

use model::shared_types::{
    common::ErrorPayload,
    controller::{ServerToControllerEvent, ServerToControllerEventType},
    host::{ServerToHostEvent, ServerToHostEventType},
};

// Helper traits for cleaner response construction

pub trait IntoHostResponse {
    fn into_response(self) -> String;
}

impl IntoHostResponse for ServerToHostEventType {
    fn into_response(self) -> String {
        serde_json::to_string(&ServerToHostEvent::Success(self)).unwrap()
    }
}

pub trait IntoControllerResponse {
    fn into_response(self) -> String;
}

impl IntoControllerResponse for ServerToControllerEventType {
    fn into_response(self) -> String {
        serde_json::to_string(&ServerToControllerEvent::Success(self)).unwrap()
    }
}

impl IntoControllerResponse for ErrorPayload {
    fn into_response(self) -> String {
        serde_json::to_string(&ServerToControllerEvent::Error(self)).unwrap()
    }
}
