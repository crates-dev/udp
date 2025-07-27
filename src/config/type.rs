use crate::*;

/// Thread-safe read-write locked server configuration.
pub type ArcRwLockServerConfig = ArcRwLock<ServerConfig>;
