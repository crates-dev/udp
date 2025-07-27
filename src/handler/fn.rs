/// Default error handler that prints errors to stderr.
///
/// # Arguments
///
/// - `String` - The error message to be printed.
///
/// # Returns
///
/// - `()` - This function does not return any value.
pub(crate) fn print_error_handle(error: String) {
    eprintln!("{}", error);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}
