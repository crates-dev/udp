use crate::*;

/// UDP server instance containing configuration and handler functions.
///
/// Provides thread-safe access to server state and operations.
#[derive(Clone)]
pub struct Server {
    /// Server configuration settings.
    pub(super) config: ArcRwLockServerConfig,
    /// List of registered handler functions.
    pub(super) func_list: ArcRwLockVecFuncBox,
}
