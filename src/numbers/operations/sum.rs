use super::super::{generate_complement, equalize, Sign, Number};

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
pub fn sum_complements(vector1: Vec<u8>, vector2: Vec<u8>, start_from_end: bool, carry: u8) -> (Vec<u8>, u8) {
    let (complement2, _) = generate_complement(vector2, 10);

    sum(vector1, complement2, start_from_end, carry, false)
}

impl Number {
        pub fn sum(&self, other: &Number) -> Number {
        let (int1, int2) = equalize(self.integer_part.clone(), other.integer_part.clone(), false);
        let (rat1, rat2) = equalize(self.rational_part.clone(), other.rational_part.clone(), true);
        let sign;
        let integer_result;
        let rational_result;

        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => {
                let (rat_result, carry) = sum_complements(rat1, rat2, true, 0);
                let (int_result, carry) = sum_complements(int1, int2, false, carry);

                if carry == 0 {
                    sign = Sign::Negative;
                    rational_result = generate_complement(rat_result, 10).0;
                    integer_result = generate_complement(int_result, 10).0;
                } else {
                    sign = Sign::Positive;
                    rational_result = rat_result;
                    integer_result = int_result;
                }
            }
            (Sign::Negative, Sign::Positive) => {
                let (rat_result, carry) = sum_complements(rat1, rat2, true, 0);
                let (int_result, carry) = sum_complements(int1, int2, false, carry);

                if carry == 0 {
                    sign = Sign::Positive;
                    rational_result = generate_complement(rat_result, 10).0;
                    integer_result = generate_complement(int_result, 10).0;
                } else {
                    sign = Sign::Negative;
                    rational_result = rat_result;
                    integer_result = int_result;
                }
            }
            _ => {
                sign = self.sign;
                let (rat_result, carry) = sum(rat1, rat2, true, 0, false);
                rational_result = rat_result;
                let (int_result, _) = sum(int1, int2, false, carry, true);
                integer_result = int_result;
            }
        }

        Number {
            integer_part: integer_result,
            rational_part: rational_result,
            sign: sign,
        }
    }
}