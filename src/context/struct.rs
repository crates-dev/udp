use crate::*;

/// Represents the internal state of the application context.
///
/// This structure holds all the data associated with a single request-response cycle,
/// including the socket, request, response, and any custom attributes.
#[derive(Clone)]
pub struct ContextData {
    /// A flag indicating whether the request handling has been aborted.
    pub(super) aborted: bool,
    /// The underlying network socket for the connection.
    pub(super) socket: Option<ArcRwLockUdpSocket>,
    /// The incoming UDP request data.
    pub(super) request: Request,
    /// The outgoing UDP response data.
    pub(super) response: Response,
    /// The client's socket address.
    pub(super) client_addr: Option<SocketAddr>,
    /// A collection of custom attributes for sharing data within the request lifecycle.
    pub(super) attributes: ThreadSafeAttributeStore,
}

/// The main application context, providing thread-safe access to request and response data.
///
/// This is a wrapper around `ContextData` that uses an `Arc<RwLock<>>` to allow
/// for shared, mutable access across asynchronous tasks.
#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<ContextData>);
