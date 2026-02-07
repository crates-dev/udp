use crate::*;

/// UDP response wrapper containing response data.
///
/// Provides a wrapper for response content.
#[derive(Clone, Debug)]
pub struct Response {
    /// The underlying response data.
    pub(super) data: ResponseData,
}
