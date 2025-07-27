use crate::*;

/// Inner context containing all UDP communication components.
///
/// Stores the socket, request/response data and additional context information.
#[derive(Clone)]
pub struct InnerContext {
    /// UDP socket wrapper with read-write lock.
    pub(crate) socket: OptionArcRwLockUdpSocket,
    /// Incoming request data.
    pub(crate) request: Request,
    /// Outgoing response data.
    pub(crate) response: Response,
    /// Remote socket address.
    pub(crate) socket_addr: OptionSocketAddr,
    /// Additional context data storage.
    pub(crate) data: HashMapArcAnySendSync,
}

/// Thread-safe context wrapper for UDP operations.
///
/// Provides synchronized access to the inner context.
#[derive(Clone)]
pub struct Context(
    /// Thread-safe reference to inner context.
    pub(super) ArcRwLock<InnerContext>,
);
