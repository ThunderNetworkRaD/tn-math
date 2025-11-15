use super::super::Number;

impl Number {
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
}