use crate::shared_types::host;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerEventWrapper {
    Host(host::ServerToHostEvent),
    Controller,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientEventWrapper {
    Host(host::HostEvent),
    Controller,
}
