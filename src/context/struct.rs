use crate::*;

#[derive(Clone, Data)]
pub struct InnerContext {
    pub(super) socket: OptionArcRwLockUdpSocket,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) socket_addr: OptionSocketAddr,
    pub(super) data: HashMapArcAnySendSync,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
