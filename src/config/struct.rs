use crate::*;

#[derive(Clone)]
pub struct ServerConfig {
    pub(crate) host: String,
    pub(crate) port: usize,
    pub(crate) buffer_size: usize,
    pub(crate) error_handle: ArcErrorHandle,
}
