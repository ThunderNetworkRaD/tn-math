pub fn equalize(vector1: Vec<u8>, vector2: Vec<u8>, ltr: bool) -> (Vec<u8>, Vec<u8>) {
    let len1 = vector1.len();
    let len2 = vector2.len();
    if len1 > len2 {
        (vector1, crate::numbers::fill::fill_with_zeros(vector2, len1, ltr))
    } else {
        (crate::numbers::fill::fill_with_zeros(vector1, len2, ltr), vector2)
    }
}