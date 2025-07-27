use crate::*;

/// Thread-safe reference-counted UDP socket.
pub type ArcUdpSocket = Arc<UdpSocket>;

/// Optional thread-safe reference-counted UDP socket.
pub type OptionArcUdpSocket = Option<ArcUdpSocket>;

/// Optional thread-safe read-write locked UDP socket.
pub type OptionArcRwLockUdpSocket = Option<ArcRwLockUdpSocket>;

/// Read guard for read-write locked UDP socket.
pub type RwLockReadGuardUdpSocket<'a> = RwLockReadGuard<'a, UdpSocket>;

/// Write guard for read-write locked UDP socket.
pub type RwLockWriteGuardUdpSocket<'a> = RwLockWriteGuard<'a, UdpSocket>;

/// Thread-safe reference-counted write guard for UDP socket.
pub type ArcRwLockWriteGuardUdpSocket<'a> = Arc<RwLockWriteGuard<'a, UdpSocket>>;

/// Optional thread-safe reference-counted write guard for UDP socket.
pub type OptionArcRwLockWriteGuardUdpSocket<'a> = Option<ArcRwLockWriteGuardUdpSocket<'a>>;

/// Thread-safe reference-counted mutex guard for UDP socket.
pub type ArcMutexGuardUdpSocket<'a> = Arc<MutexGuard<'a, UdpSocket>>;

/// Optional thread-safe reference-counted mutex guard for UDP socket.
pub type OptionArcMutexGuardUdpSocket<'a> = Option<ArcMutexGuardUdpSocket<'a>>;

/// Optional socket host IP address.
pub type OptionSocketHost = Option<IpAddr>;

/// Optional socket port number.
pub type OptionSocketPort = Option<u16>;

/// Optional socket address (IP + port).
pub type OptionSocketAddr = Option<SocketAddr>;
