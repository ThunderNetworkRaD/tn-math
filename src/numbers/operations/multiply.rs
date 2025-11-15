use super::super::{Number, minify, equalize, Sign, sum};

impl Number {
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
}