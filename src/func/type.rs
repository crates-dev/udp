use crate::*;

pub type FuncBox = Box<dyn Func + Send + 'static>;
pub type ArcRwLockVecFuncBox = ArcRwLock<Vec<FuncBox>>;
