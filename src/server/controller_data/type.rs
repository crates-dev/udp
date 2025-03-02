use crate::*;

pub type RwLockWriteControllerData<'a> = RwLockWriteGuard<'a, ControllerData>;
pub type RwLockReadControllerData<'a> = RwLockReadGuard<'a, ControllerData>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) socket: OptionArcRwLockUdpSocket,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
    pub(super) addr: OptionSocketAddr,
}

#[derive(Clone, Debug)]
pub struct ArcRwLockControllerData(pub(super) ArcRwLock<ControllerData>);
