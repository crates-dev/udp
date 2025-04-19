use crate::*;

#[derive(Clone, Debug)]
pub struct ArcRwLockUdpSocket(pub(super) ArcRwLock<UdpSocket>);
