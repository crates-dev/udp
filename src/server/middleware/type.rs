use crate::*;

pub type FuncArcLock = AsyncArcRwLock<Vec<FuncBox>>;
