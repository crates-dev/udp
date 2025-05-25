use crate::*;

#[derive(Clone, Data)]
pub struct ServerConfig {
    pub(super) host: String,
    pub(super) port: usize,
    pub(super) buffer_size: usize,
    pub(super) error_handle: ArcErrorHandle,
}
