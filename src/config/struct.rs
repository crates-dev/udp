use crate::*;

/// Server configuration settings.
///
/// Contains all necessary parameters to configure a UDP server.
#[derive(Clone)]
pub struct ServerConfig {
    /// Server host address.
    pub(crate) host: String,
    /// Server port number.
    pub(crate) port: usize,
    /// Maximum buffer size for incoming packets.
    pub(crate) buffer_size: usize,
    /// Error handling callback function.
    pub(crate) error_handle: ArcErrorHandle,
}
