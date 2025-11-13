/// Pad a byte vector with zeroes until it reaches a specified total length.
///
/// This function mutates `vector` in place by repeatedly inserting `0` bytes
/// until `vector.len() == total_length`.
///
/// Behavior:
/// - If `vector.len() >= total_length`, the function will panic.
/// - If `begin` is `Some(true)`, zeros are inserted at the front of the vector
///   (index 0) until the desired length is reached.
/// - If `begin` is `Some(false)` or `None`, zeros are appended to the end of the
///   vector. `None` is treated the same as `Some(false)`.
///
/// Panics:
/// - Panics if `vector.len() >= total_length`.
///
/// Performance:
/// - Inserting at the front of a `Vec` is O(n) per insert, so using
///   `begin = Some(true)` may be O(n * m) overall (quadratic) where `m` is the
///   number of inserted zeros. If many front insertions are required, consider
///   using a `VecDeque<u8>` or preallocating a new vector and copying.
///
/// Examples:
/// ```rust
/// # use tn_math::numbers::fill_with_zeros; // adjust path as appropriate
/// let mut v = vec![1u8, 2, 3];
/// fill_with_zeros(&mut v, 5, None);
/// assert_eq!(v, vec![1, 2, 3, 0, 0]);
///
/// let mut v2 = vec![4u8];
/// fill_with_zeros(&mut v2, 3, Some(true));
/// assert_eq!(v2, vec![0, 0, 4]);
/// ```
pub fn fill_with_zeros(mut vector: Vec<u8>, total_length: usize, ltr: bool) -> Vec<u8> {
    while vector.len() < total_length {
        if ltr {
            vector.insert(0, 0);
        } else {
            vector.push(0);
        }
    }

    vector
}

pub fn unfill_zeros(mut vector: Vec<u8>, begin: bool) {
    while vector[if begin { 0 } else { vector.len() - 1 }] == 0 && vector.len() > 1 {
        if begin {
            vector.remove(0);
        } else {
            vector.pop();
        }
    }
}