use crate::*;

/// Thread-safe reference-counted read-write lock wrapper.
pub type ArcRwLock<T> = Arc<RwLock<T>>;
