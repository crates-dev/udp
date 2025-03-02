use crate::*;

pub type AsyncArcRwLock<T> = Arc<RwLock<T>>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLock<ServerConfig>,
    pub(super) func_list: FuncListArcLock,
    pub(super) tmp: ArcRwLock<Tmp>,
}
