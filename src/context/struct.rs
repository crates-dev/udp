use crate::*;

#[derive(Clone, Lombok)]
pub struct InnerContext {
    pub(super) socket: OptionArcRwLockUdpSocket,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
    pub(super) socket_addr: OptionSocketAddr,
    pub(super) data: HashMapArcAnySendSync,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
