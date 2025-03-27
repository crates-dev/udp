use crate::*;

pub type RwLockWriteContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadContext<'a> = RwLockReadGuard<'a, InnerContext>;

#[derive(Clone, Lombok)]
pub struct InnerContext {
    pub(super) socket: OptionArcRwLockUdpSocket,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
    pub(super) socket_addr: OptionSocketAddr,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
