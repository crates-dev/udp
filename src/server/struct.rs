use crate::*;

#[derive(Clone)]
pub struct Server {
    pub(super) config: ArcRwLockServerConfig,
    pub(super) func_list: ArcRwLockVecFuncBox,
}
