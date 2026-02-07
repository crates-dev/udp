use crate::*;

/// Represents the internal, mutable state of the UDP server.
///
/// This struct consolidates all the core components required for the server to operate,
/// including configuration, hooks, and task panic hooks for extending functionality.
/// It is not intended to be used directly by end-users, but rather wrapped within the `Server` struct
/// for thread-safe access.
#[derive(Clone)]
pub(crate) struct ServerData {
    /// Stores the server's configuration settings, such as address, port, and buffer size.
    pub(super) server_config: ServerConfigData,
    /// A collection of request hooks that are invoked for each incoming request.
    pub(super) hook: ServerHookList,
    /// A collection of task panic hooks that are invoked when a panic occurs during request processing.
    pub(super) task_panic: ServerHookList,
    /// The read error hooks for server operations.
    pub(super) read_error: ServerHookList,
}

/// The primary server structure that provides a thread-safe interface to the server's state.
///
/// This struct acts as a public-facing wrapper around an `Arc<RwLock<ServerData>>`.
/// It allows multiple parts of the application to safely share and modify the server's
/// configuration and state across different threads and asynchronous tasks.
#[derive(Clone)]
pub struct Server(pub(super) ArcRwLock<ServerData>);
