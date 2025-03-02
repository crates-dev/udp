use crate::*;

pub trait AsyncFuncWithoutPin<Fut>:
    Fn(ArcRwLockControllerData) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

pub trait Func:
    Fn(ArcRwLockControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    + Send
    + Sync
    + 'static
{
}
