use crate::numbers::{equalize, sum::{sum, sum_complements}};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative
}

/// Represents a number with integer and rational parts, along with its sign.
/// Integer part is stored as a vector of digits (u8), where the (index - 1) represents the power of 10.
/// Rational part is also stored as a vector of digits (u8), where the index represents the negative power of 10.
pub struct Number {
    pub integer_part: Vec<u8>,
    pub rational_part: Vec<u8>,
    pub sign: Sign,
}

impl Number {
    pub fn minify(&mut self) {
        while self.integer_part.len() > 1 && self.integer_part[0] == 0 {
            self.integer_part.remove(0);
        }

        while self.rational_part.len() > 1 && *self.rational_part.last().unwrap() == 0 {
            self.rational_part.pop();
        }
    }

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

                integer_result = int_result;
                rational_result = rat_result;

                if carry != 0 {
                    sign = Sign::Positive;
                } else {
                    sign = Sign::Negative;
                }
            }
            (Sign::Negative, Sign::Positive) => {
                let (rat_result, carry) = sum_complements(rat2, rat1, true, 0);
                let (int_result, carry) = sum_complements(int2, int1, false, carry);

                integer_result = int_result;
                rational_result = rat_result;

                if carry != 0 {
                    sign = Sign::Positive;
                } else {
                    sign = Sign::Negative;
                }
            }
            _ => {
                sign = self.sign;
                let (rat_result, carry) = sum(rat1, rat2, true, 0);
                rational_result = rat_result;
                integer_result = sum(int1, int2, false, carry).0;
            }
        }

        Number {
            integer_part: integer_result,
            rational_part: rational_result,
            sign: sign,
        }
    }
}