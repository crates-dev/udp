use crate::*;

/// Type alias for response data (byte vector).
pub type ResponseData = Vec<u8>;

/// Result type for response operations.
pub type ResponseResult = Result<(), ResponseError>;
