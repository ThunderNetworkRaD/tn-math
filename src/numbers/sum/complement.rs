/**
This is used to generate the complement of a number represented as a vector of digits in a given base.
Useful for subtraction operations in arbitrary-precision arithmetic.
 */
pub fn generate_complement(vector: Vec<u8>, base: u8) -> (Vec<u8>, u8) {
    let mut complement = Vec::new();
    let mut carry = 1; // Start with 1 for the addition of 1 in the complement

    for &digit in vector.iter().rev() {
        let mut comp_digit = (base - 1) - digit + carry;
        if comp_digit >= base {
            comp_digit -= base;
            carry = 1;
        } else {
            carry = 0;
        }
        complement.insert(0, comp_digit);
    }

    if carry == 1 {
        complement.insert(0, carry);
    }

    (complement, carry)
}

/// This function sums two vectors of digits represented in a given base.
///
/// It first generates the complement of the second vector (i.e. the value of the
/// second vector subtracted from the base - 1) and then sums the original
/// vector with the complement of the second vector. This is useful for
/// subtraction operations in arbitrary-precision arithmetic.
///
/// The function returns a tuple containing the resulting vector of digits and a
/// carry value. If the carry value is greater than 0, it means that the
/// resulting vector is one digit longer than the original vectors.
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
pub fn sum_complements(vector1: Vec<u8>, vector2: Vec<u8>, rtl: bool, carry: u8) -> (Vec<u8>, u8) {
    let (complement2, _) = generate_complement(vector2, 10);

    let mut sum = crate::numbers::sum::sum(vector1, complement2, rtl, carry);

    if sum.1 > 0 {
        sum.0.remove(0);
        (sum.0, sum.1)
    } else {
        (generate_complement(sum.0, 10).0, sum.1)
    }
}