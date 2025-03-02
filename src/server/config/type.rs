use crate::*;

#[derive(Clone, Debug, Lombok)]
pub struct ServerConfig {
    pub(super) host: String,
    pub(super) port: usize,
    pub(super) log_dir: String,
    pub(super) log_size: usize,
    pub(super) buffer_size: usize,
    pub(super) interval_millis: usize,
    pub(super) print: bool,
}
