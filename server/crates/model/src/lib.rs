pub mod local_types;
mod shared;

pub mod shared_types {
    pub mod host {
        pub use crate::shared::common_types::*;
        pub use crate::shared::host_types::*;
    }
}
