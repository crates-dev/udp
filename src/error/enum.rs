/// Represents errors that can occur at the server level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerError {
    /// An error occurred while trying to bind to a UDP socket.
    UdpBind(String),
    /// An unknown or unexpected error occurred.
    Unknown(String),
    /// An error occurred while reading a UDP request.
    UdpRead(String),
    /// Other error.
    Other(String),
}

/// Represents errors related to response operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseError {
    /// An error occurred while sending a response.
    SendError(String),
    /// The socket is not available.
    SocketNotAvailable,
    /// The address is not available.
    AddressNotAvailable,
    /// An unknown or unexpected error occurred.
    Unknown,
}

/// Represents errors related to request operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestError {
    /// An error occurred while reading a request.
    ReadError(String),
    /// The request buffer is too small.
    BufferTooSmall,
    /// An unknown or unexpected error occurred.
    Unknown,
}
