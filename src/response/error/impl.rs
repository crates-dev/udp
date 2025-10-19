use crate::*;

/// Standard error implementation for ResponseError.
impl StdError for ResponseError {}

/// Display implementation for ResponseError.
impl Display for ResponseError {
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
            Self::ResponseError(data) => write!(f, "Response Error{}{}", COLON_SPACE, data),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
