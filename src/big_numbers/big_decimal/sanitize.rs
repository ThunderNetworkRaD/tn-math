use crate::big_numbers::big_decimal::BigDecimal;

fn remove_zeros(arr: &mut Vec<u8>, from_start: bool) {
    if from_start {
        while arr.first() == Some(&0) {
            arr.remove(0);
        }
    } else {
        while arr.last() == Some(&0) {
            arr.pop();
        }
    }
}

pub fn sanitize(mut x: BigDecimal) -> BigDecimal {
    remove_zeros(&mut x.integer_part, true);
    remove_zeros(&mut x.fractional_part, false);

    BigDecimal {
        integer_part: x.integer_part,
        fractional_part: x.fractional_part,
    }
}

impl BigDecimal {
    pub fn sanitize(&self) -> Self {
        sanitize(self.clone())
    }
}