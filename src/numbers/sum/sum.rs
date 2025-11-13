/// Sums two vectors of digits in a given base. The vectors must be of the same length.
///
/// # Arguments
///
/// * `vector1`: The first vector of digits.
/// * `vector2`: The second vector of digits.
/// * `rtl`: Whether to sum the vectors from right to left (true) or left to right (false).
///
/// # Returns
///
/// A tuple containing the resulting vector of digits and a carry value.
pub fn sum(vector1: Vec<u8>, vector2: Vec<u8>, rtl: bool, mut carry: u8) -> (Vec<u8>, u8) {
    if vector1.len() != vector2.len() {
        panic!("Vectors must be of the same length");
    }

    let mut result = Vec::new();    
    for mut i in if rtl { (vector1.len() + 1)..0 } else { 0..vector1.len() } {
        if rtl { i -= 1; }
        let mut sum = vector1[i] + vector2[i] + carry;
        if sum > 9 {
            carry = 1;
            sum -= 10;
        } else {
            carry = 0;
        }
        result.insert(0, sum);
    }

    (result, carry)
}