use crate::*;

/// Thread-safe wrapper for UDP socket with read-write lock.
///
/// Provides synchronized access to UDP socket operations.
#[derive(Clone, Debug)]
pub struct ArcRwLockUdpSocket {
    /// Underlying UDP socket with read-write lock.
    pub(super) socket: ArcRwLock<UdpSocket>,
}
