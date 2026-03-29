pub mod local_types;
mod shared;

pub mod shared_types {
    pub mod common {
        pub use crate::shared::common_types::{ErrorPayload, PingPayload, PongPayload};
    }

    pub mod host {
        pub use crate::shared::host_types::{HostEvent, ServerToHostEvent};
    }

    pub mod controller {
        pub use crate::shared::controller_types::{ControllerEvent, ServerToControllerEvent};
    }
}
