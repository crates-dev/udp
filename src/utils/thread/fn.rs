/// Gets the number of available threads for parallel processing.
///
/// # Returns
///
/// - `usize` - Number of available threads, defaults to 1 if detection fails.
pub fn get_thread_count() -> usize {
    match std::thread::available_parallelism() {
        Ok(count) => count.get(),
        Err(_) => 1,
    }
}
