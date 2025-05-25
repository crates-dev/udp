use crate::*;

#[derive(Clone, Data)]
pub struct Server {
    pub(super) config: ArcRwLockServerConfig,
    pub(super) func_list: ArcRwLockVecFuncBox,
}
