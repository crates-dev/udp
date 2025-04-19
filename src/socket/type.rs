use crate::*;

pub type ArcUdpSocket = Arc<UdpSocket>;
pub type OptionArcUdpSocket = Option<ArcUdpSocket>;
pub type OptionArcRwLockUdpSocket = Option<ArcRwLockUdpSocket>;
pub type RwLockReadGuardUdpSocket<'a> = RwLockReadGuard<'a, UdpSocket>;
pub type RwLockWriteGuardUdpSocket<'a> = RwLockWriteGuard<'a, UdpSocket>;
pub type ArcRwLockWriteGuardUdpSocket<'a> = Arc<RwLockWriteGuard<'a, UdpSocket>>;
pub type OptionArcRwLockWriteGuardUdpSocket<'a> = Option<ArcRwLockWriteGuardUdpSocket<'a>>;
pub type ArcMutexGuardUdpSocket<'a> = Arc<MutexGuard<'a, UdpSocket>>;
pub type OptionArcMutexGuardUdpSocket<'a> = Option<ArcMutexGuardUdpSocket<'a>>;
pub type OptionSocketHost = Option<IpAddr>;
pub type OptionSocketPort = Option<u16>;
pub type OptionSocketAddr = Option<SocketAddr>;
