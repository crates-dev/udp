use crate::*;

pub trait AsyncFuncWithoutPin<Fut>:
    Fn(ControllerData) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

pub trait Func:
    Fn(ControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    + Send
    + Sync
    + 'static
{
}
