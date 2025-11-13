use crate::numbers::{equalize, sum::{sum, complement::sum_complements}};
use std::fmt::Debug;

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

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_str = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        let integer_str: String = self.integer_part.iter().rev().map(|d| d.to_string()).collect();
        let rational_str: String = self.rational_part.iter().map(|d| d.to_string()).collect();

        if self.rational_part.is_empty() {
            write!(f, "{}{}", sign_str, integer_str)
        } else {
            write!(f, "{}{}.{}", sign_str, integer_str, rational_str)
        }
    }
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

                if carry == 0 {
                    sign = Sign::Positive;
                    rational_result = super::sum::complement::generate_complement(rat_result, 10).0;
                    integer_result = super::sum::complement::generate_complement(int_result, 10).0;
                } else {
                    sign = Sign::Negative;
                    rational_result = rat_result;
                    integer_result = int_result;
                }
            }
            (Sign::Negative, Sign::Positive) => {
                let (rat_result, carry) = sum_complements(rat1, rat2, true, 0);
                let (int_result, carry) = sum_complements(int1, int2, false, carry);

                if carry == 0 {
                    sign = Sign::Positive;
                    rational_result = super::sum::complement::generate_complement(rat_result, 10).0;
                    integer_result = super::sum::complement::generate_complement(int_result, 10).0;
                } else {
                    sign = Sign::Negative;
                    rational_result = rat_result;
                    integer_result = int_result;
                }
            }
            _ => {
                sign = self.sign;
                let (rat_result, carry) = sum(rat1, rat2, true, 0);
                rational_result = rat_result;
                let (mut int_result, carry) = sum(int1, int2, false, carry);
                if carry != 0 {
                    int_result.insert(0, carry);
                }
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