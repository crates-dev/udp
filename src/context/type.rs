use crate::*;

pub type RwLockWriteContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadContext<'a> = RwLockReadGuard<'a, InnerContext>;
pub type ArcAnySendSync = Arc<dyn Any + Send + Sync>;
pub type HashMapArcAnySendSync = HashMap<String, ArcAnySendSync>;
