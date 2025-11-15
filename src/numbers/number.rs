use super::*;

/// Represents a number with integer and rational parts, along with its sign.
/// Integer part is stored as a vector of digits (u8), where the (index - 1) represents the power of 10.
/// Rational part is also stored as a vector of digits (u8), where the index represents the negative power of 10.
#[derive(Clone)]
pub struct Number {
    pub integer_part: Vec<u8>,
    pub rational_part: Vec<u8>,
    pub sign: Sign,
}

pub fn is_greater_than(vector1: &Vec<u8>, vector2: &Vec<u8>, begin: bool) -> bool {
    if vector1.len() != vector2.len() {
        return vector1.len() > vector2.len();
    }

    for i in 0..vector1.len() {
        let index = if begin { i } else { vector1.len() - 1 - i };
        if vector1[index] != vector2[index] {
            return vector1[index] > vector2[index];
        }
    }

    false
}