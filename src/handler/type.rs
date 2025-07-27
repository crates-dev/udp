use crate::*;

/// Boxed handler function with Send bound.
pub(crate) type FuncBox = Box<dyn Func + Send + 'static>;

/// Thread-safe vector of handler functions.
pub(crate) type ArcRwLockVecFuncBox = ArcRwLock<Vec<FuncBox>>;

/// Thread-safe error handler with Send+Sync bounds.
pub(crate) type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
