use crate::*;

#[derive(Clone, Data)]
pub struct Server {
    pub(super) cfg: ArcRwLockServerConfig,
    pub(super) func_list: ArcRwLockVecFuncBox,
    pub(super) tmp: ArcRwLock<Tmp>,
}
