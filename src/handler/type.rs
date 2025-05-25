use crate::*;

pub(crate) type FuncBox = Box<dyn Func + Send + 'static>;
pub(crate) type ArcRwLockVecFuncBox = ArcRwLock<Vec<FuncBox>>;
pub(crate) type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
