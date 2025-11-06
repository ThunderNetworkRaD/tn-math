pub fn integer_part_sum(bigger_vector: Vec<u8>, smaller_vector: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut carry: u8 = 0;
    for i in 0..bigger_vector.len() {
        let x = bigger_vector[bigger_vector.len() - 1 - i];
        let y = if i < smaller_vector.len() { smaller_vector[smaller_vector.len() - 1 - i] } else { 0 };
        let sum = x + y + carry;

        // Determine if there is a carry for the next digit
        if sum > 9 {
            carry = 1;
        } else {
            carry = 0;
        }

        // Store the result digit
        result.insert(0, sum % 10);
    }

    // If there's a carry left after the last addition, add it to the front
    if carry > 0 {
        result.insert(0, carry);
    }

    result
}