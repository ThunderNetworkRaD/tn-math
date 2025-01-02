use crate::big_numbers::big_decimal::BigDecimal;

impl BigDecimal {
    pub fn sum(&self, other: &BigDecimal) -> BigDecimal {
        let mut integer_part: Vec<u8> = Vec::new();
        let mut fractional_part: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;
        
        self.sanitize();
        other.sanitize();

        let (longer, shorter) = if self.fractional_part.len() >= other.fractional_part.len() {
            (
                self.fractional_part.clone(),
                other.fractional_part.clone(),
            )
        } else {
            (
                other.fractional_part.clone(),
                self.fractional_part.clone(),
            )
        };
        
        for i in (0..longer.len()).rev() {
            let sum = longer.get(i).unwrap_or(&0) + shorter.get(i).unwrap_or(&0) + carry;
            carry = sum / 10;
            fractional_part.insert(0,sum % 10);
        }

        // TODO: Split here for multithreading

        let (longer, shorter) = if self.integer_part.len() >= other.integer_part.len() {
            (
                self.integer_part.clone(),
                other.integer_part.clone(),
            )
        } else {
            (
                other.integer_part.clone(),
                self.integer_part.clone(),
            )
        };

        for i in 1..(longer.len() + 1) {
            let sum = longer.get(longer.len() - &i).unwrap_or(&0) + shorter.get(longer.len() - &i).unwrap_or(&0) + carry;
            carry = sum / 10;
            integer_part.insert(0, sum % 10);
        }

        if carry != 0 {
            integer_part.insert(0, carry);
        }

        BigDecimal {
            integer_part,
            fractional_part,
        }
    }
}