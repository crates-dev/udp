use crate::*;

/// Optional thread-safe read-write locked UDP socket.
pub type OptionArcRwLockUdpSocket = Option<ArcRwLockUdpSocket>;

/// Optional socket address (IP + port).
pub type OptionSocketAddr = Option<SocketAddr>;

/// Optional socket host IP address.
pub type OptionSocketHost = Option<std::net::IpAddr>;

/// Optional socket port number.
pub type OptionSocketPort = Option<u16>;
