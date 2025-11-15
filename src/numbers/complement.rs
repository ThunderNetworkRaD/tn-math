/**
This is used to generate the complement of a number represented as a vector of digits in a given base.
Useful for subtraction operations in arbitrary-precision arithmetic.
 */
pub fn generate_complement(vector: Vec<u8>, base: u8) -> (Vec<u8>, u8) {
    let mut complement = Vec::new();
    let mut carry = 1;

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