#[derive(PartialEq, Debug, Clone)]
pub enum Sign {
    Positive,
    Negative
}

pub struct Number {
    pub integer_part: Vec<u8>,
    pub rational_part: Vec<u8>,
    pub sign: Sign,
}

impl Number {
    pub fn sum(&self, other: &Number) -> Number {
        if (self.sign == Sign::Negative && other.sign == Sign::Positive) ||
           (self.sign == Sign::Positive && other.sign == Sign::Negative) {
            unimplemented!("Subtraction not implemented yet");
        } else {
            let rational_sum = crate::numbers::sum::rational_part_sum(self.rational_part.clone(), other.rational_part.clone());
            let integer_sum = crate::numbers::sum::integer_part_sum(self.integer_part.clone(), other.integer_part.clone());
            let integer_sum = if rational_sum.1 > 0 {
                crate::numbers::sum::integer_part_sum(integer_sum, vec![rational_sum.1])
            } else {
                integer_sum
            };
            
            Number {
                integer_part: integer_sum,
                rational_part: rational_sum.0,
                sign: Sign::Positive,
            }
        }
    }
}