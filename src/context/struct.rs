use crate::*;

#[derive(Clone)]
pub struct InnerContext {
    pub(crate) socket: OptionArcRwLockUdpSocket,
    pub(crate) request: Request,
    pub(crate) response: Response,
    pub(crate) socket_addr: OptionSocketAddr,
    pub(crate) data: HashMapArcAnySendSync,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
