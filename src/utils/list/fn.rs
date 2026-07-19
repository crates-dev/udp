/// Removes trailing zeros from a byte vector.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - Mutable reference to the byte vector.
///
/// # Returns
///
/// - `Vec<u8>` - New vector with trailing zeros removed.
pub fn remove_trailing_zeros(data: &mut Vec<u8>) -> Vec<u8> {
    if let Some(last_non_zero_pos) = data.iter().rposition(|&x| x != 0) {
        data.truncate(last_non_zero_pos + 1);
    } else {
        data.clear();
    }
    data.clone()
}
