use crate::*;

/// Type alias for thread-safe attribute storage.
///
/// This type is used to store arbitrary data that can be shared across
/// different parts of the application in a thread-safe manner.
pub type ThreadSafeAttributeStore = HashMap<String, Arc<dyn Any + Send + Sync>>;

/// Type alias for types that implement Any + Send + Sync + Clone.
///
/// This is a trait alias for types that can be stored in the attribute store
/// and safely shared across threads.
pub trait AnySendSyncClone: Any + Send + Sync + Clone {}
impl<T> AnySendSyncClone for T where T: Any + Send + Sync + Clone {}
