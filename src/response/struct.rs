use crate::*;

/// UDP response wrapper containing response data.
///
/// Provides thread-safe access to response content.
#[derive(Clone, Debug)]
pub struct Response(
    /// The underlying response data.
    pub(super) ResponseData,
);
