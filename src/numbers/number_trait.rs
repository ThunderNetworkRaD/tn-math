use crate::numbers::number::{Number, Sign};

pub trait NumberTrait {
    fn digits(&self) -> Vec<u8>;
    fn to_number(&self) -> Number;
}

impl NumberTrait for usize {
    fn digits(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        let mut n = *self;
        while n > 0 {
            v.push((n % 10).try_into().unwrap());
            n /= 10;
        }
        v.reverse();
        v
    }

    fn to_number(&self) -> Number {
        Number {
            integer_part: self.digits(),
            rational_part: Vec::new(),
            sign: Sign::Positive,
        }
    }
}

impl NumberTrait for i32 {
    fn digits(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        let mut n = *self;
        while n > 0 {
            v.push((n % 10).try_into().unwrap());
            n /= 10;
        }
        v.reverse();
        v
    }

    fn to_number(&self) -> Number {
        Number {
            integer_part: self.digits(),
            rational_part: Vec::new(),
            sign: if *self > 0 { Sign::Positive } else { Sign::Negative },
        }
    }
}