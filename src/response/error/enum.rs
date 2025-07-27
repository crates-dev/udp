/// Error types for response operations.
#[derive(Debug)]
pub enum ResponseError {
    /// Represents a response error with message.
    ResponseError(String),
    /// Unknown response error.
    Unknown,
}
