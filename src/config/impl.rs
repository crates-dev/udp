use crate::*;

/// Default implementation for ServerConfig.
impl Default for ServerConfig {
    /// Creates a default server configuration.
    ///
    /// # Returns
    ///
    /// - `ServerConfig` - New config with default values.
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_LISTEN_PORT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            error_handle: Arc::new(print_error_handle),
        }
    }
}
