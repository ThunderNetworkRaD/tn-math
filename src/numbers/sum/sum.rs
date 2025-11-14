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
pub fn sum(vector1: Vec<u8>, vector2: Vec<u8>, start_from_end: bool, mut carry: u8, sum_last_carry: bool) -> (Vec<u8>, u8) {
    let length = vector1.len();
    if length != vector2.len() {
        panic!("Vectors must be of the same length");
    }

    let mut result = Vec::new();   

    for mut i in 0..vector1.len() {
        if start_from_end { i = length - 1 - i }

        let mut sum = vector1[i] + vector2[i] + carry;
        if sum > 9 {
            carry = 1;
            sum -= 10;
        } else {
            carry = 0;
        }

        if start_from_end {
            result.insert(0, sum);
        } else {
            result.push(sum);
        }
    }

    if sum_last_carry && carry != 0 {
        println!("Adding final carry {}", carry);
        if start_from_end {
            result.insert(0, carry);
        } else {
            result.push(carry);
        }
        carry = 0;
    }

    (result, carry)
}