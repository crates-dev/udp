use crate::*;

impl ArcRwLockUdpSocket {
    pub fn from(arc_rw_lock_socket: ArcRwLock<UdpSocket>) -> Self {
        Self(arc_rw_lock_socket)
    }

    pub fn from_socket(socket: UdpSocket) -> Self {
        Self(Arc::new(RwLock::new(socket)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadGuardUdpSocket {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteGuardUdpSocket {
        self.0.write().await
    }
}
