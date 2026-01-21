/// Error types for server operations.
#[derive(Debug)]
pub enum ServerError {
    /// Represents a TCP bind error with message.
    TcpBindError(String),
    /// Unknown server error.
    Unknown,
}
