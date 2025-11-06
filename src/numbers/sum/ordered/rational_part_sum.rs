pub fn rational_part_sum(bigger_vector: Vec<u8>, smaller_vector: Vec<u8>) -> (Vec<u8>, u8) {
    let mut result = Vec::new();
    let length_diff: usize = bigger_vector.len() - smaller_vector.len();

    for i in (bigger_vector.len() - length_diff + 1 .. bigger_vector.len()).rev() {
        result.push(bigger_vector[i]);
    }

    let mut carry: u8 = 0;
    for i in (0..smaller_vector.len()).rev() {
        let sum = bigger_vector[i + length_diff] + smaller_vector[i] + carry;

        // Determine if there is a carry for the next digit
        if sum > 9 {
            carry = 1;
        } else {
            carry = 0;
        }

        // Store the result digit
        result.insert(0, sum % 10);
    }

    (result, carry)
}