use crate::*;

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_LISTEN_PORT,
            log_dir: DEFAULT_LOG_DIR.to_owned(),
            log_size: DEFAULT_LOG_FILE_SIZE,
            buffer_size: DEFAULT_BUFFER_SIZE,
            inner_print: DEFAULT_INNER_PRINT,
            inner_log: DEFAULT_INNER_LOG,
        }
    }
}
