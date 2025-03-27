use crate::*;

pub type ArcRwLockTmp = ArcRwLock<Tmp>;

#[derive(Clone, Lombok)]
pub struct Tmp {
    pub(crate) log: Log,
}
