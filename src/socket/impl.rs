use crate::*;

/// Implementation of ArcRwLockUdpSocket methods.
///
/// Provides construction and access methods for thread-safe UDP socket.
impl ArcRwLockUdpSocket {
    /// Creates a new instance from existing ArcRwLock<UdpSocket>.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLock<UdpSocket>` - Existing socket with read-write lock.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockUdpSocket` - New wrapper instance.
    pub fn from(arc_rw_lock_socket: ArcRwLock<UdpSocket>) -> Self {
        Self(arc_rw_lock_socket)
    }

    /// Creates a new instance from raw UdpSocket.
    ///
    /// # Arguments
    ///
    /// - `UdpSocket` - Raw UDP socket.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockUdpSocket` - New wrapper instance.
    pub fn from_socket(socket: UdpSocket) -> Self {
        Self(Arc::new(RwLock::new(socket)))
    }

    /// Acquires a read lock on the socket.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuardUdpSocket` - Read guard for the socket.
    pub async fn get_read_lock(&self) -> RwLockReadGuardUdpSocket {
        self.0.read().await
    }

    /// Acquires a write lock on the socket.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuardUdpSocket` - Write guard for the socket.
    pub async fn get_write_lock(&self) -> RwLockWriteGuardUdpSocket {
        self.0.write().await
    }
}
