use crate::*;

/// Trait for error handling functions.
pub trait ErrorHandle: Fn(String) {}

/// Trait for async handler functions without Pin requirement.
///
/// # Generic Parameters
///
/// - `Fut` - Future type returned by the function.
pub trait AsyncFuncWithoutPin<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

/// Trait for general handler functions.
///
/// Wraps async functions in Pin<Box<dyn Future>>.
pub trait Func:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
{
}
