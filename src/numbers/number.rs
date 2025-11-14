use crate::numbers::{equalize, sum::{complement::sum_complements, sum}};
use std::{fmt::Debug, vec};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative
}

/// Represents a number with integer and rational parts, along with its sign.
/// Integer part is stored as a vector of digits (u8), where the (index - 1) represents the power of 10.
/// Rational part is also stored as a vector of digits (u8), where the index represents the negative power of 10.
#[derive(Clone)]
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

pub fn positive_get_digit(integer_part: &Vec<u8>, rational_part: &Vec<u8>, index: usize) -> u8 {
    let total_length = integer_part.len() + rational_part.len();
    let index_from_end = total_length - index - 1;

    if index_from_end < rational_part.len() {
        rational_part[index_from_end]
    } else {
        integer_part[index_from_end - rational_part.len()]
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
                    sign = Sign::Negative;
                    rational_result = super::sum::complement::generate_complement(rat_result, 10).0;
                    integer_result = super::sum::complement::generate_complement(int_result, 10).0;
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
                    sign = Sign::Negative;
                    rational_result = super::sum::complement::generate_complement(rat_result, 10).0;
                    integer_result = super::sum::complement::generate_complement(int_result, 10).0;
                } else {
                    sign = Sign::Positive;
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

    pub fn multiply(&self, other: &Number) -> Number {
        let self_rational_length = self.rational_part.len();
        let other_rational_length = other.rational_part.len();

        let self_length = self_rational_length + self.integer_part.len();
        let other_length = other_rational_length + other.integer_part.len();

        let mut result = vec![0; self_length + other_length + 1];

        for i in (0..other_length).rev() {
            let mut carry: u16 = 0;            
            for j in (0..self_length).rev() {
                let digit = 
                    self.positive_get_digit(j) as u16 *
                    other.positive_get_digit(i) as u16 +
                    carry;               

                carry = digit / 10;

                result[i + j] = (digit % 10) as u8;
            }

            let mut k = i + self_length;
            while carry > 0 {
                let digit = result[k] as u16 + carry;
                result[k] = (digit % 10) as u8;
                carry = digit / 10;
                k += 1;
            }
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        result.reverse();

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        let decimals = self_rational_length + other_rational_length;
        let result_len = result.len();
        let split = result_len.saturating_sub(decimals);
        let integer_part = if split == 0 {
            vec![0]
        } else {
            let mut integer_part = result[..split].to_vec();
            integer_part.reverse();
            integer_part
        };

        let rational_part = if self_rational_length + other_rational_length == 0 {
            vec![]
        } else if decimals > result_len {
            let mut pad = vec![0; decimals - result_len];
            pad.extend(result);
            pad
        } else {
            result[split..].to_vec()
        };

        Number {
            integer_part: integer_part,
            rational_part: rational_part,
            sign: if self.sign == other.sign { Sign::Positive } else { Sign::Negative },
        }
    }

    pub fn positive_get_digit(&self, index: usize) -> u8 {
        positive_get_digit(&self.integer_part, &self.rational_part, index)
    }
}