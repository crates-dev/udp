use crate::*;

/// Standard error implementation for ServerError.
impl StdError for ServerError {}

/// Display implementation for ServerError.
impl Display for ServerError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UdpBind(data) => write!(f, "UDP bind error: {data}"),
            Self::Unknown(data) => write!(f, "Unknown error: {data}"),
            Self::UdpRead(data) => write!(f, "UDP read error: {data}"),
            Self::Other(data) => write!(f, "Other error: {data}"),
        }
    }
}

/// Standard error implementation for ResponseError.
impl StdError for ResponseError {}

/// Display implementation for ResponseError.
impl Display for ResponseError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendError(data) => write!(f, "Send error: {data}"),
            Self::SocketNotAvailable => write!(f, "Socket not available"),
            Self::AddressNotAvailable => write!(f, "Address not available"),
            Self::Unknown => write!(f, "Unknown response error"),
        }
    }
}

/// Standard error implementation for RequestError.
impl StdError for RequestError {}

/// Display implementation for RequestError.
impl Display for RequestError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter` - Formatter for the output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadError(data) => write!(f, "Read error: {data}"),
            Self::BufferTooSmall => write!(f, "Buffer too small"),
            Self::Unknown => write!(f, "Unknown request error"),
        }
    }
}
