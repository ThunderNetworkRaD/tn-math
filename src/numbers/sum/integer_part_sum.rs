pub fn integer_part_sum(vector1: Vec<u8>, vector2: Vec<u8>) -> Vec<u8> {
    if vector1.len() >= vector2.len() {
        crate::numbers::sum::ordered::integer_part_sum(vector1, vector2)
    } else {
        crate::numbers::sum::ordered::integer_part_sum(vector2, vector1)
    }
}