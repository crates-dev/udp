use crate::*;

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_LISTEN_PORT,
            buffer_size: DEFAULT_BUFFER_SIZE,
            error_handle: Arc::new(print_error_handle),
        }
    }
}
