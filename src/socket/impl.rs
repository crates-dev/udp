use crate::*;

/// Implementation of ArcRwLockUdpSocket methods.
impl ArcRwLockUdpSocket {
    /// Creates a new instance from existing `ArcRwLock<UdpSocket>`.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLock<UdpSocket>` - Existing socket with read-write lock.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockUdpSocket` - New wrapper instance.
    pub fn from(socket: ArcRwLock<UdpSocket>) -> Self {
        Self { socket }
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
        Self {
            socket: arc_rwlock(socket),
        }
    }

    /// Acquires a read lock on the socket.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<UdpSocket>` - Read guard for the socket.
    pub async fn get_read_lock(&self) -> RwLockReadGuard<'_, UdpSocket> {
        self.socket.read().await
    }

    /// Acquires a write lock on the socket.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<UdpSocket>` - Write guard for the socket.
    pub async fn get_write_lock(&self) -> RwLockWriteGuard<'_, UdpSocket> {
        self.socket.write().await
    }
}
