mod generated {
    pub mod host {
        include!("generated/host.types.v1.rs");
    }

    pub mod controller {
        include!("generated/controller.types.v1.rs");
    }
}

pub mod host {
    //! Host-related types and services
    pub use super::generated::host::*;
}

pub mod controller {
    //! Controller-related types and services
    pub use super::generated::controller::*;
}
