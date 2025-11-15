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

pub fn positive_get_digit(integer_part: &Vec<u8>, rational_part: &Vec<u8>, index: usize, from_end: bool) -> u8 {
    let integer_len = integer_part.len();
    let rational_len = rational_part.len();

    if !from_end {
        if index < integer_len {
            // Parte intera è MSB-first → ribaltiamo
            return integer_part[integer_len - 1 - index];
        } else {
            // Parte razionale è LSB-first → accesso diretto
            return rational_part[index - integer_len];
        }
    } else {
    if index < rational_len {
        // Parte razionale è MSB-first → ribaltiamo
        rational_part[rational_len - 1 - index]
    } else {
        // Parte intera è LSB-first → accesso diretto
        integer_part[index - rational_len]
        }
    }
}

pub fn minify(vector: &mut Vec<u8>, begin: bool) {
    while vector.len() > 1 && vector[if begin { 0 } else { vector.len() - 1 }] == 0 {
        if begin {
            vector.remove(0);
        } else {
            vector.pop();
        }
    }
}

impl Number {
    pub fn minify(&mut self) {
        minify(&mut self.integer_part, false);
        minify(&mut self.rational_part, true);
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
        let mut result = vec![0; self.integer_part.len() + self.rational_part.len() + other.integer_part.len() + other.rational_part.len()];
        let mut index = 0;

        for i in 0..(other.rational_part.len() + other.integer_part.len()) {
            let mut temp_result = vec![];

            let mut carry = 0;            
            for j in 0..(self.rational_part.len() + self.integer_part.len()) {
                let mut digit = 
                    self.positive_get_digit(j, true) *
                    other.positive_get_digit(i, true) +
                    carry;
                
                carry = digit / 10;
                digit = digit % 10;

                temp_result.push(digit);
            }

            if carry != 0 {
                temp_result.push(carry);
            }

            for _ in 0..index {
                temp_result.insert(0, 0);
            }

            index += 1;

            let (equalized_result, temp_result) = equalize(result, temp_result.clone(), false);

            result = sum(equalized_result, temp_result, false, 0, true).0;
        }

        let (rational_part, integer_part) = result.split_at(self.rational_part.len() + other.rational_part.len());
        let mut integer_part = integer_part.to_vec();
        let mut rational_part = rational_part.to_vec();
        minify(&mut integer_part, false);
        minify(&mut rational_part, true);
        rational_part.reverse();

        Number {
            integer_part: integer_part.to_vec(),
            rational_part: rational_part.to_vec(),
            sign: if self.sign == other.sign { Sign::Positive } else { Sign::Negative },
        }
    }

    pub fn divide(&self, _other: &Number, mut _precision: usize) -> Number {
        unimplemented!();
        /* 
        let mut integer_result = vec![];
        let mut rational_result = vec![];

        let mut number = self.clone();
        let mut other = other.clone();

        let other_rational_length = other.rational_part.len();
        other.shift_rationals_to_integers(other_rational_length);
        number.shift_rationals_to_integers(other_rational_length);

            let mut actual_part = Vec::new();
            let mut counter = 0;
            while !is_greater_than(&actual_part, &other.integer_part, true) {
                actual_part.insert(0, number.positive_get_digit(counter, false));
                counter += 1;
            }


        Number {
            integer_part: integer_result,
            rational_part: rational_result,
            sign: Sign::Positive,
        }
         */
    }

    pub fn shift_rational_to_integer(&mut self) {
        let digit = if self.rational_part.is_empty() {
            0
        } else {
            self.rational_part.remove(0)
        };
        if digit == 0 && self.integer_part.is_empty() {
            return;
        }
        self.integer_part.insert(0, digit);
    }

    pub fn shift_rationals_to_integers(&mut self, count: usize) {
        for _ in 0..count {
            self.shift_rational_to_integer();
        }
    }

    pub fn shift_integer_to_rational(&mut self) {
        let digit = if self.integer_part.is_empty() {
            0
        } else {
            self.integer_part.remove(0)
        };
        if digit == 0 && self.rational_part.is_empty() {
            return;
        }
        self.rational_part.insert(0, digit);
    }

    pub fn shift_integers_to_rationals(&mut self, count: usize) {
        for _ in 0..count {
            self.shift_integer_to_rational();
        }
    }

    pub fn positive_get_digit(&self, index: usize, from_end: bool) -> u8 {
        positive_get_digit(&self.integer_part, &self.rational_part, index, from_end)
    }
}