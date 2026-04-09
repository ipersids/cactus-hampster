pub mod local_types;
mod shared;

pub mod shared_types {
    pub mod common {
        pub use crate::shared::common_types::{
            ControllerInputPayload, CreateSessionPayload, ErrorPayload, GameStartedPayload,
            JoinSessionPayload, JoinSuccessPayload, PingPayload, PlayerInfo, PlayerInputPayload,
            PlayerJoinedPayload, PlayerLeftPayload, PongPayload, SessionCreatedPayload,
            StartGamePayload,
        };
    }

    pub mod host {
        pub use crate::shared::host_types::{
            HostEvent, HostEventType, ServerToHostEvent, ServerToHostEventType,
        };
    }

    pub mod controller {
        pub use crate::shared::controller_types::{
            ControllerEvent, ControllerEventType, ServerToControllerEvent,
            ServerToControllerEventType,
        };
    }
}
