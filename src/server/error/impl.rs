use super::r#enum::ServerError;
use crate::*;

/// Standard error implementation for ServerError.
impl StdError for ServerError {}

/// Display implementation for ServerError.
impl Display for ServerError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// - `&fmt::Formatter` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TcpBindError(data) => write!(f, "Tcp bind error{}{}", COLON_SPACE, data),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
