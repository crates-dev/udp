use crate::*;

/// Default server hook.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DefaultServerHook;

/// Represents the hooks for managing the server's lifecycle, specifically for waiting and shutting down.
#[derive(Clone)]
pub struct ServerControlHook {
    /// A hook that returns a future, which completes when the server's main task finishes.
    /// This is typically used to wait for the server to stop accepting connections before
    /// the application exits.
    pub(crate) wait_hook: Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>,
    /// A hook that, when called, initiates a graceful shutdown of the server.
    /// This will stop the server from accepting new connections and allow existing ones
    /// to complete.
    pub(crate) shutdown_hook: Arc<dyn Fn() -> SendableAsyncTask<()> + Send + Sync>,
}

/// Represents the state associated with a single connection handler.
///
/// This struct encapsulates the necessary context for processing a connection,
/// including a reference to the network socket. It is created
/// for each connection and passed to the relevant handlers.
#[derive(Clone)]
pub(crate) struct HandlerState {
    /// A reference to the underlying network socket for the connection.
    pub(super) socket: ArcRwLockUdpSocket,
}
