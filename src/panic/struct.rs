/// Represents data captured from a panic.
///
/// This structure holds information about a panic that occurred during
/// request processing, including the panic message and location if available.
#[derive(Clone, Debug)]
pub struct PanicData {
    /// The panic message.
    pub(super) message: String,
    /// The location where the panic occurred (file and line).
    pub(super) location: Option<String>,
}
